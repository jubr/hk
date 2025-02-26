# Getting Started

A tool for running hooks on files in a git repository.

> [!CAUTION]
> This is a work in progress. Everything about this project including the docs is probably going to change.

## Installation

Use [mise-en-place](https://github.com/jdx/mise) to install hk (you'll also need the `pkl` cli):

```sh
mise use hk pkl
hk --version
```

:::tip
mise-en-place integrates well with hk. Features common in similar git-hook managers like dependency management, task dependencies, and env vars can be provided by mise.

See [mise integration](/mise_integration) for more information.
:::

Or install from source with `cargo`:

```sh
cargo install hk
```

## Project Setup

Use `hk generate` to generate a `hk.pkl` file:

```sh
hk generate
```

## `hk.pkl`

This will generate a `hk.pkl` file in the root of the repository, here's an example `hk.pkl` with eslint and prettier hooks:

```pkl
amends "https://hk.jdx.dev/v0/hk.pkl"
import "https://hk.jdx.dev/v0/builtins/prettier.pkl"

`pre-commit` {
    // hooks can be manually defined
    ["eslint"] {
        // the files to run the hook on, if no files are matched, the hook will be skipped
        // this will filter the staged files and return the subset matching these globs
        glob = new { "*.js"; "*.ts" }
        // a command that returns non-zero to fail the step
        check = "eslint {{files}}"
    }
    // hooks can also be specified with the builtins pkl library
    ["prettier"] = new prettier.Prettier {}
}
```

See [configuration](/configuration) for more information on the `hk.pkl` file.

## Usage

Inside a git repository with a `hk.pkl` file, run:

```sh
hk install
```

This will install the hooks for the repository like `pre-commit` and `pre-push` if they are defined in `hk.pkl`. Running `git commit` would now run the `pre-commit` steps defined above in our example.

### `core.hooksPath`

As an alternative to using `hk install`, you can run `git config --local core.hooksPath .hooks` to use the hooks defined in the `.hooks` directory of the repository:

```sh
#!/bin/sh
hk run pre-commit
```

## Running Hooks

To explicitly run the hooks without going through git, use the `hk run` command.

```sh
hk run pre-commit
```

This will run the `pre-commit` hooks for the repository. This will run against all files that are staged for commit. To run against all files in the repository, use the `--all` flag.

```sh
hk run pre-commit --all
```

To run a specific step, use the `--step` flag.

```sh
hk run pre-commit --step eslint
```
