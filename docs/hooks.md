# Hooks

The following describes the behavior of git hooks that hk supports.

## `pre-commit`

Runs when `git commit` is run before the commit is created. By default this will run "fix"[0] commands instead of "check"[1] which by convention
may edit files. While automatically formatting code with fixers is convenient, it may result in slower behavior as hk may need to lock files
if 2 steps are to edit the same file.

* Stashes any untracked/unstaged changes (disable with [`HK_STASH=0`](/configuration#hk-stash))
* Gathers list of files with staged changes (or all files if running `hk run pre-commit --all`)
* Runs steps in parallel up to [`HK_JOBS`](/configuration#hk-jobs) at a time, with caveats:
  * `exclusive = true` steps will wait until all previous steps finished and block later steps from starting
  * if any step has any dependencies, hk will wait for them to complete before starting
  * hk will create read/write locks for each file to check/fix in the steps
  * if "fix" is set (default behavior) _and_ multiple steps in the same group[2] are to edit the same file, hk will do one of the following:
    * if `stomp = true`, hk will grab read locks instead of write locks for the "fix" step. Use this if the tool itself has its own locking
      behavior or you simply don't care if the files may write by multiple fix steps at the same time.
    * if `check_first = true` on the step, hk will run the "check" commands first with a read lock, if that fails, it will run the "fix" command with a write lock
    * if `check_first = false` on the step, hk will run the "fix" command with write locks, blocking other steps from running
    * modified files are added to the git index
  * if "check" is set (because the step does not have a "fix" command, [`HK_FIX=0`](/configuration#hk-fix) is set, or `hk run pre-commit --check`), hk runs all steps in parallel. They should not be modifying files so this should be safe to do.
  * untracked/unstaged changes are unstashed
  * commit is allowed to run if no check/fix steps failed

## `pre-push`

Runs when `git push` is run before `git push` sends the changes to the remote repository.

TODO

[0]: "fix" commands may edit files. This requires some locking behavior which may impact performance so that steps don't conflict with each other.
[1]: "check" commands may not edit files and can run in max parallelism—so long as 0 fix steps are also running. Note that this is enforced only
    by convention. hk will not prevent you from editing files in a check step and does not watch the repo for modifications for performance reasons.
[2]: A group is a collection of steps separated by steps with `exclusive = true`.
