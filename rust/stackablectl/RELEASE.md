# Release Process

The release of `stackablectl` is currently mostly done manually. This means the following steps need
to be done:

1. Ensure your local `main` branch is up-to-date and then proceed to checkout a new branch using
   `git checkout -b chore/release-stackablectl-X.Y.Z`.
2. Update both the Cargo.toml and CHANGELOG.md file to the new version `X.Y.Z`.
3. Add the relevant changes from the changelog to a new release notes partial under
   `docs/stackablectl/partials/release-notes`.
4. Update various files by running the following xtask `cargo xtask gen-man` and
   `make regenerate-nix`. This is also automatically done if pre-commit is enabled.
5. Push the changes and raise a PR.
6. Merge the PR onto `main` and then proceed to tag the appropriate commit using
   `git tag -s stackablectl-Y.Y.Z -m stackablectl-Y.Y.Z`.
7. Building the artifacts and creating the release on GitHub is fully automated from this point
   onward.
