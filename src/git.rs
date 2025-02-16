use std::{path::PathBuf, thread, time::Duration};

use git2::{Commit, Repository, StashFlags, StatusOptions, StatusShow, Tree};
use itertools::Itertools;
use miette::Result;
use miette::{miette, Context, IntoDiagnostic};

use crate::env;

pub struct Git {
    repo: Repository,
    stash_diff: Option<Vec<u8>>,
    root: PathBuf,
}

impl Git {
    pub fn new() -> Result<Self> {
        let repo = Repository::open(".")
            .into_diagnostic()
            .wrap_err("failed to open repository")?;
        Ok(Self {
            root: repo.workdir().unwrap().to_path_buf(),
            repo,
            stash_diff: None,
        })
    }

    fn head_tree(&self) -> Result<Tree<'_>> {
        let head = self
            .repo
            .head()
            .into_diagnostic()
            .wrap_err("failed to get head")?;
        let head = head
            .peel_to_tree()
            .into_diagnostic()
            .wrap_err("failed to peel head to tree")?;
        Ok(head)
    }

    fn head_commit(&self) -> Result<Commit<'_>> {
        let head = self
            .repo
            .head()
            .into_diagnostic()
            .wrap_err("failed to get head")?;
        let commit = head
            .peel_to_commit()
            .into_diagnostic()
            .wrap_err("failed to peel head to commit")?;
        Ok(commit)
    }

    fn head_commit_message(&self) -> Result<String> {
        let commit = self.head_commit()?;
        let message = commit
            .message()
            .ok_or(miette!("failed to get commit message"))?;
        Ok(message.to_string())
    }

    pub fn all_files(&self) -> Result<Vec<PathBuf>> {
        let head = self.head_tree()?;
        let mut files = Vec::new();
        head.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            if let Some(name) = entry.name() {
                let path = if root.is_empty() {
                    PathBuf::from(name)
                } else {
                    PathBuf::from(root).join(name)
                };
                if path.exists() {
                    files.push(path);
                }
            }
            git2::TreeWalkResult::Ok
        })
        .into_diagnostic()
        .wrap_err("failed to walk tree")?;
        Ok(files)
    }

    // pub fn intent_to_add_files(&self) -> Result<Vec<PathBuf>> {
    //     // let added_files = self.added_files()?;
    //     // TODO: get this to work, should be the equivalent of `git diff --name-only --diff-filter=A`
    //     Ok(vec![])
    // }

    pub fn staged_files(&self) -> Result<Vec<PathBuf>> {
        let mut status_options = StatusOptions::new();
        status_options.show(StatusShow::Index);
        let statuses = self
            .repo
            .statuses(Some(&mut status_options))
            .into_diagnostic()
            .wrap_err("failed to get statuses")?;
        let paths = statuses
            .iter()
            .filter_map(|s| s.path().map(PathBuf::from))
            .filter(|p| p.exists())
            .collect_vec();
        Ok(paths)
    }

    pub fn unstaged_files(&self) -> Result<Vec<PathBuf>> {
        let mut status_options = StatusOptions::new();
        status_options
            .include_untracked(true)
            .show(StatusShow::Workdir);
        let statuses = self
            .repo
            .statuses(Some(&mut status_options))
            .into_diagnostic()
            .wrap_err("failed to get statuses")?;
        let paths = statuses
            .iter()
            .filter_map(|s| s.path().map(PathBuf::from))
            .collect_vec();
        Ok(paths)
    }

    pub fn stash_unstaged(&mut self, force: bool) -> Result<()> {
        // Skip stashing if there's no initial commit yet or auto-stash is disabled
        if (!force && !*env::HK_AUTO_STASH) || self.repo.head().is_err() {
            return Ok(());
        }

        // TODO: if any intent_to_add files exist, run `git rm --cached -- <file>...` then `git add --intent-to-add -- <file>...` when unstashing
        // let intent_to_add = self.intent_to_add_files()?;
        // see https://github.com/pre-commit/pre-commit/blob/main/pre_commit/staged_files_only.py
        if self.unstaged_files()?.is_empty() {
            return Ok(());
        }

        if let Ok(msg) = self.head_commit_message() {
            if msg.contains("Merge") {
                return Ok(());
            }
        }
        self.stash_diff = self.build_diff()?;
        if self.stash_diff.is_none() {
            return Ok(());
        }

        let mut checkout_opts = git2::build::CheckoutBuilder::new();
        self.repo
            .checkout_head(Some(&mut checkout_opts))
            .into_diagnostic()
            .wrap_err("failed to checkout head")?;

        Ok(())
    }

    fn build_diff(&self) -> Result<Option<Vec<u8>>> {
        // essentially: git diff-index --ignore-submodules --binary --exit-code --no-color --no-ext-diff (git write-tree)
        let tree = self.head_tree()?;
        let diff = self
            .repo
            .diff_tree_to_index(Some(&tree), None, None)
            .into_diagnostic()
            .wrap_err("failed to get diff")?;
        let mut diff_bytes = vec![];
        diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
            if delta.new_file().path().unwrap().exists() {
                diff_bytes.extend(line.content());
            }
            true
        })
        .into_diagnostic()
        .wrap_err("failed to print diff")?;
        if diff_bytes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(diff_bytes))
        }
    }

    pub fn add(&mut self, pathspecs: &[&str]) -> Result<()> {
        let pathspecs = pathspecs
            .iter()
            .map(|p| p.replace(self.root.to_str().unwrap(), ""))
            .collect_vec();
        trace!("adding files: {:?}", &pathspecs);
        let mut index = self
            .repo
            .index()
            .into_diagnostic()
            .wrap_err("failed to get index")?;
        index
            .add_all(&pathspecs, git2::IndexAddOption::DEFAULT, None)
            .into_diagnostic()
            .wrap_err("failed to add files to index")?;
        index
            .write()
            .into_diagnostic()
            .wrap_err("failed to write index")?;
        Ok(())
    }

    pub fn pop_stash(&mut self) -> Result<()> {
        let Some(diff) = self.stash_diff.take() else {
            return Ok(());
        };

        let diff = git2::Diff::from_buffer(&diff).into_diagnostic()?;
        let mut apply_opts = git2::ApplyOptions::new();
        self.repo
            .apply(&diff, git2::ApplyLocation::WorkDir, Some(&mut apply_opts))
            .into_diagnostic()
            .wrap_err("failed to apply diff")?;

        Ok(())
    }

    // pub fn reset_index(&mut self) -> Result<()> {
    //     let head = self.repo.head()?;
    //     let tree = head.peel_to_tree()?;
    //         .reset(&tree.into_object(), git2::ResetType::Mixed, None
    // }
}
