null :=
space := $(null) $(null)

.ONESHELL:
SHELL := /bin/bash

.SILENT:

.DEFAULT:
	joined=$(subst $(space),-,$(MAKECMDGOALS));

	if [ "$$joined" = "$@" ]
	then
		echo "No rule for target '$@'.";
		exit 1;
	fi

	if ! $(MAKE) -q $$joined >/dev/null 2>&1
	then
		echo "No rule for target '$(MAKECMDGOALS)'.";
		exit 1;
	fi
	
	$(MAKE) -s --no-print-directory $$joined;


test-code:
	cargo test -- --nocapture --color=always

test-format:
	cargo +nightly fmt --all -- --check

test-coverage-get:
	coverage=$$(cargo llvm-cov -- --nocapture | grep '^TOTAL' | awk '{print $$10}');

	if [ -z "$$coverage" ]
	then
		echo "Tests failed.";
		exit 1;
	fi

	echo "$$coverage";

test-coverage-export:
	if [ -z "$(export)" ]
	then
		EXPORT_PATH="./coverage.lcov";
	else
		EXPORT_PATH="$(export)";
	fi;

	cargo llvm-cov --lcov -- --nocapture > $$EXPORT_PATH 2>/dev/null;
