# `hk run pre-commit`

- **Usage**: `hk run pre-commit [FLAGS]`
- **Aliases**: `pc`

Sets up git hooks to run hk

## Flags

### `-a --all`

Run on all files instead of just staged files

### `-f --fix`

Run fix command instead of run command This is the default behavior unless HK_FIX=0

### `-r --run`

Run run command instead of fix command

### `--stash`

Force stashing even if it's disabled via HK_AUTO_STASH
