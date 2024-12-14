# Contributing to hyperdrive-rs

## Documentation

We strive for verbose and readable comments and docstrings.
We use [katex](https://www.npmjs.com/package/katex) to render LaTeX math in the docs.

## Contributor git workflow

We follow a standard [feature branch rebase workflow](https://www.atlassian.com/git/tutorials/comparing-workflows/feature-branch-workflow) that prioritizes short PRs with isolated improvements.

### Reviewers

Please check each item **before approving** the pull request. While going
through the checklist, it is recommended to leave comments on items that are
referenced in the checklist to make sure that they are reviewed.

- Are there new or updated unit or integration tests?
- Do new fuzz tests use the maximum tolerable number of runs
  (aka without increasing the total test runtime)?
- Do the tests cover the happy paths?
- Do the tests cover the unhappy paths?
- If matching Solidity behavior, are there differential fuzz tests that
  ensure that Rust matches Solidity?

## Publishing a release

For code owners who wish to publish a release, make sure to follow these guidelines:

1. Merge a PR that edits the package version -- use the same version label for all files.
  * `Cargo.toml` to specify the new Rust semver version. You'll need to edit the workspace version as well as the package versions for `fixedpointmath`, `hyperdrive-wrappers`, and `hyperdrive-math`.
  * `pyproject.toml` to specify the **same** Python semver version.
  * `setup.py` to specify the **same** Python semver version.
2. From the updated `main` branch, make a new tag with `git tag vX.Y.Z`.
3. Push that tag to the remote repository with `git push --tags`.
4. Go to the `releases` tab in Github and add the new tag as a release.
5. Click the "Generate Release Notes" button to generate release notes.

## Debugging
To point tests to a local Anvil instance, use the environment variable `HYPERDRIVE_ETHEREUM_URL`.
