# Contributing to the repository

When contributing to this repository, please first discuss the change you wish to make via the [issue tracker](https://github.com/analogdevicesinc/adi-adxl355-rs/issues) before making a pull request.

Please note we have a [code of conduct](./CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

The [adi-adxl355-rs repository](https://github.com/analogdevicesinc/adi-adxl355-rs) is a workspace of crates all under the Apache License version 2.0.

Any pull requests will be covered by this license.

## Pull Request Checklist

1. Commit message includes a "Signed-off-by: [name] < email >" to the commit message.
   This ensures you have the rights to submit your code, by agreeing to the [Developer Certificate of Origin](https://developercertificate.org/). If you can not agree to the DCO, don't submit a pull request, as we can not accept it.
   Check [the git documentation](https://git-scm.com/docs/git-commit#Documentation/git-commit.txt--s) on how to add sign-off to your commits.
2. Commit should be "atomic", i.e. : should do one thing only. A pull requests should only contain multiple commits if that is required to fix the bug or implement the feature.
3. Commits should have good commit messages. Check out [The git Book](https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project) for some pointers, and tools to use.
4. The project must build for the `thumbv7em-none-eabihf` target and `aarch64-unknown-linux-gnu` target. This is checked on every pull request by the continuous integration system, things that fail to build can not be merged.

## Pull Request Process

1. Make a fork, if you are not sure on how to make a fork, check out [GitHub help](https://help.github.com/en/github/getting-started-with-github/fork-a-repo)
2. Make a Pull Request, if you are not sure on how to make a pull request, check out [GitHub help](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request-from-a-fork)
3. Before a Pull Request can be merged, it must be reviewed by at least one reviewer, and tested on the devices used for the examples. If you have tested it, you can indicate that in your commit message.
