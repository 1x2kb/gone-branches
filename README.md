# gone-branches

# Installing
`cargo install --path=.`
This will use cargo to install the crate locally on your machine. The crate name is `gbranches`.

# Executing
`git fetch origin --prune && git branch -v | gbranches | xargs git branch -D`

`git fetch origin --prune`
This will update the local state to that of the remote. Removing any branches that no longer exist on the remote. However, pruning does not remove local branches so you can still freely `git checkout` on any pruned branch that also already exists on your local machine.

`git branch -v` prints verbose information about branches on the local machine. The part the program is interested in, is the [gone] part of that output which is present when the remote branch of a local branch is deleted.

`gbranches` this repositories code. Parses the output from the `git branch -v` command and prints out the gone branches. Filtering out any extra text excluding the branch name.

`xargs git branch -D` finally pass the list of branches that are gone to the git delete command.

**Note:** there is no confirmation of deletion and this will immediately delete the branches. Use with caution.
