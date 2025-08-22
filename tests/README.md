# Integration Tests

First of all, thanks for your intention on contributing to this project.

In this crate we aim for stability and ease of use for all the macros the crate
declares, we want to be helpful not a burden. To accomplish this we need to test
every part of the crate.

Since this crate is completely a wrapper, we don't focus on build errors, we instead
focus on what should work. Error in itself are implicit and can be expected if there
is a misuse.

Since this crate is completely a wrapper, we don't really focus on build errors,
even tho error testing is implemented in some unitary testing, that tests whether
the inner functions return correct values, not error texts or whether they should
appear if the crate is miss-used.

**This separate test crate is integration testing, for unitary testing see `src/tests`.**

## DSL Test Macro

We provide a macro to test this project, which generates a random-port binding
HTTP server and sends a request to it, this for the sake of testing an actix_web
handler.

An example use for the macro is the following
```rust
/* incomplete handler example */
async fn handler() -> impl Responder;

test_http_endpoint! {
	test handler as test_handler
	with request {
		head: get /path/to/handler?with="parameter";
		headers: {
			Key: "value"
		}
		body: {
			"An expr representing the body."
		}
	}
	and expect response {
		head: 500;
		headers: {
			Key: "value"
		}
		body: {
			"An expr representing the body."
		}
	}
}
```

Note that you don't need to specify all the headers, this is just inclusive checking.
You can not include headers or body at all, the result of including a header is just
generating an assert about that header with that key being `Some()` and having the same value.

The only required value is the head for both the request and the response.

## File System

All the files are spread but with a feature describing name, all they have in common
is that they all declare `common` as a module in the top. The `common` module can be
modified if there is something you think many other tests may use.

## Adding Tests

Adding integration tests must be always done when a new publicly exposed feature is added,
you may create a new file that includes the `common` module if you wish to use the DSL test
macro or add any common test code. This project uses `cargo-llvm-cov`, and expects a coverage
of at least 80%.

## Running The Tests

This project uses make for some command recipes. You can run `make test_code` and it will
test the application using the correct parameters.

It is not recommended to rely on `cargo test` because parameters may change depending
on needs.
