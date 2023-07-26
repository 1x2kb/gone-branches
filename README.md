# gone-branches


# What Is This?
This is a small code base written in Rust to delete the local branches that have also been removed on the remote. It achieves this by running a few git commands.

`git fetch origin --prune` updates local to the current state in the remote.
`git branch -v` Prints out information regarding the git branches on the local. This includes important information such as the branch name and hash. Additionally, it includes a label '[gone]'.
Each branch returned by the git branch command is filtered based on that gone label. Those branches get passed to the final git command for deletion.
`git branch -D ...`

# Installing
`cargo install --path=.`
This will use cargo to install the crate locally on your machine. The crate name is `gbranches`.

# Usage
Once installed this can be run simply in the terminal using the crate name. In this case `gbranches`.
This will automatically execute against the current directory.