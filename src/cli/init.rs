use std::path::PathBuf;

use crate::{version, Result};

/// Sets up git hooks to run hk
#[derive(Debug, clap::Args)]
#[clap()]
pub struct Init {}

impl Init {
    pub async fn run(&self) -> Result<()> {
        let hk_file = PathBuf::from("hk.pkl");
        let version = version::version();
        let hook_content = format!(
            r#"
// amends "package://hk.jdx.dev/hk@0.1.0#/hk.pkl"
amends "pkl/hk.pkl"
import "pkl/builtins.pkl"

min_hk_version = "{version}"

// example git hooks are defined below
//
// pre_commit {{
//     // "prelint" here is simply a name to define the step
//     ["prelint"] {{
//         // if a step has a "run" script it will execute that
//         run = "mise run prelint"
//         exclusive = true // ensures that the step runs in isolation
//     }}
//     // everything from here to postlint is run in parallel
//     ["pkl"] {{
//         glob = new {{ "*.pkl" }}
//         run = "pkl eval {{staged_files}} >/dev/null"
//     }}
//     // predefined formatters+linters
//     ["cargo-check"] = new builtins.CargoCheck {{}}
//     ["cargo-fmt"] = new builtins.CargoFmt {{}}
//     ["eslint"] = new builtins.Eslint {{}}
//     ["prettier"] = new builtins.Prettier {{
//         glob = new {{ "*.js"; "*.ts" }} // override the default globs
//     }}
//     ["postlint"] {{
//         run = "mise run postlint"
//         exclusive = true
//     }}
// }}
//
// // instead of pre-commit, you can instead define pre-push hooks
// pre_push {{
//     ["eslint"] = new builtins.Eslint {{}}
// }}
//
// // TODO
// commit_msg {{
// }}
//
// // TODO
// prepare_commit_msg {{
// }}
//
// // TODO
// update {{
// }}
"#
        );
        xx::file::write(hk_file, hook_content.trim_start())?;
        Ok(())
    }
}
