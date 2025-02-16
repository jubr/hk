use std::{path::PathBuf, thread, time::Duration};

use git2::{Repository, StashFlags, StatusOptions, StatusShow};
use itertools::Itertools;
use miette::Result;
use miette::{Context, IntoDiagnostic};

pub struct Git {
    repo: Repository,
    stash_id: Option<git2::Oid>,
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
            stash_id: None,
        })
    }

    pub fn all_files(&self) -> Result<Vec<PathBuf>> {
        let head = self
            .repo
            .head()
            .into_diagnostic()
            .wrap_err("failed to get head")?;
        let head = head
            .peel_to_tree()
            .into_diagnostic()
            .wrap_err("failed to peel head to tree")?;
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

    pub fn stash_unstaged(&mut self) -> Result<()> {
        // Skip stashing if there's no initial commit yet
        if self.repo.head().is_err() {
            return Ok(());
        }

        if !self.unstaged_files()?.is_empty() {
            self.push_stash(3)?;
        }
        Ok(())
    }

    fn push_stash(&mut self, retries: u32) -> Result<()> {
        {
            let head = self
                .repo
                .head()
                .into_diagnostic()
                .wrap_err("failed to get head")?;
            let head = head
                .peel_to_commit()
                .into_diagnostic()
                .wrap_err("failed to peel head to commit")?;
            let head = head.message().unwrap();
            if head.contains("Merge") {
                return Ok(());
            }
        }

        let stasher = self
            .repo
            .signature()
            .into_diagnostic()
            .wrap_err("failed to get signature")?;
        let stash_flags = StashFlags::KEEP_INDEX | StashFlags::INCLUDE_UNTRACKED;
        match self
            .repo
            .stash_save(&stasher, "hk pre-commit stash", Some(stash_flags))
        {
            Ok(stash_id) => {
                self.stash_id = Some(stash_id);
            }
            Err(e) => {
                warn!("failed to save stash: {e:?}");
                if retries > 0 {
                    thread::sleep(Duration::from_secs(1));
                    return self.push_stash(retries - 1);
                }
            }
        }
        Ok(())
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
        if self.stash_id.is_none() {
            return Ok(());
        };

        // TODO: figure out how to pop the stash with untracked files using git2
        duct::cmd!("git", "stash", "pop")
            .run()
            .into_diagnostic()
            .wrap_err("failed to pop stash")?;
        // let stash_id = self.stash_id.unwrap();

        // // Find the stash index by its ID
        // let mut stash_index = None;
        // self.repo.stash_foreach(|index, _, id| {
        //     if *id == stash_id {
        //         stash_index = Some(index);
        //         false // stop iteration
        //     } else {
        //         true // continue iteration
        //     }
        // })?;

        // if let Some(index) = stash_index {
        //     self.repo.stash_pop(index, None)?;
        //     self.stash_id = None;
        // }

        Ok(())
    }

    // pub fn reset_index(&mut self) -> Result<()> {
    //     let head = self.repo.head()?;
    //     let tree = head.peel_to_tree()?;
    //         .reset(&tree.into_object(), git2::ResetType::Mixed, None
    // }
}
