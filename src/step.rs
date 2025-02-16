use crate::tera;
use crate::Result;
use crate::{git::Git, glob};
use ensembler::CmdLineRunner;
use itertools::Itertools;
use miette::miette;
use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fmt, sync::Arc};

use serde_with::{serde_as, OneOrMany};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct FileLocks {
    pub read: Option<String>,
    pub write: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub r#type: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde_as(as = "Option<OneOrMany<_>>")]
    pub glob: Option<Vec<String>>,
    pub run: String,
    pub run_all: Option<String>,
    pub root: Option<PathBuf>,
    pub exclusive: bool,
    pub file_locks: Option<FileLocks>,
    #[serde_as(as = "Option<OneOrMany<_>>")]
    pub stage: Option<Vec<String>>,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileKind {
    Text,
    Binary,
    Executable,
    NotExecutable,
    Symlink,
    NotSymlink,
}

impl Step {
    pub async fn run(&self, ctx: &StepContext) -> Result<()> {
        let mut tctx = tera::Context::default();
        let staged_files = if let Some(glob) = &self.glob {
            let matches = glob::get_matches(glob, &ctx.staged_files)?;
            if matches.is_empty() {
                debug!("no matches for step: {:?}", self.name);
                return Ok(());
            }
            matches
        } else {
            ctx.staged_files.clone()
        };
        tctx.with_globs(self.glob.as_ref().unwrap_or(&vec![]));
        tctx.with_staged_files(staged_files.as_ref());
        let pr = self.build_pr();
        let mut run_all = false;
        let run = match (&self.run_all, ctx.all_files) {
            (Some(run), true) => {
                run_all = true;
                tera::render(run, &tctx).unwrap()
            }
            _ => tera::render(&self.run, &tctx).unwrap(),
        };
        pr.set_message(run.clone());
        CmdLineRunner::new("sh")
            .arg("-c")
            .arg(run)
            .with_pr(pr.clone())
            .execute()
            .await
            .into_diagnostic()?;
        if let Some(stage) = &self.stage {
            let stage = stage
                .iter()
                .map(|s| tera::render(s, &tctx).unwrap())
                .collect_vec();
            pr.set_message(format!("staging {}", stage.join(" ")));
            let mut repo = Git::new()?;
            let matches = glob::get_matches(&stage, &staged_files)?;
            let staged_files = matches.iter().map(|f| f.to_str().unwrap()).collect_vec();
            if !staged_files.is_empty() {
                repo.add(&staged_files)?;
            }
        }
        if run_all || staged_files.is_empty() {
            pr.finish_with_message("done".to_string());
        } else {
            pr.finish_with_message(format!(
                "{} file{}",
                staged_files.len(),
                if staged_files.len() == 1 { "" } else { "s" }
            ));
        }
        Ok(())
    }

    fn build_pr(&self) -> Arc<Box<dyn clx::SingleReport>> {
        let mpr = clx::MultiProgressReport::get();
        mpr.add(&self.name)
    }
}

pub struct StepContext {
    pub all_files: bool,
    pub staged_files: Vec<PathBuf>,
}
