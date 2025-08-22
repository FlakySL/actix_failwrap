# Unit Tests

First of all, thanks for your intention on contributing to this project.

In this crate we aim for stability and ease of use for all the macros the crate
declares, we want to be helpful not a burden. To accomplish this we need to test
every part of the crate.

**This test module is unitary testing, for integration testing see `../../tests`.**

## File System

Since unitary testing can access private project elements, unlike the integration tests
there is no `common` crate, as in reality everything needed may already be found
in the crate itself. All the tests are spread in multiple files per sub module.

## Adding Tests

Adding unit tests must always be done when a helper function is added to the code or something
is being abstracted, this project uses `cargo-llvm-cov` and expects a coverage of at least 80%
which is mostly accomplished with unitary testing.

## Running The Tests

This project uses make for some command recipes. You can run `make test_code` and it will
test the application using the correct parameters.

It is not recommended to rely on `cargo test` because parameters may change depending
on needs.
