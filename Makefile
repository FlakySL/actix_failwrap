.SILENT: test_code
.SILENT: test_format
.SILENT: coverage

test_code:
	cargo test -- --nocapture --color=always

test_format:
	cargo +nightly fmt --all -- --check

coverage:
	coverage=$$(cargo llvm-cov -- --nocapture --test-threads=1 --color=always 2>/dev/null | grep '^TOTAL' | awk '{print $$10}'); \
	echo "coverage=$$coverage";

ifdef export
	if [ "$(export)" == "_" ]; then \
		EXPORT_PATH="./coverage.lcov"; \
	else \
		EXPORT_PATH="$(export)"; \
	fi; \
	echo "export_path=$$EXPORT_PATH"; \
	cargo llvm-cov --lcov -- --nocapture --test-threads=1 --color=always > $$EXPORT_PATH 2>/dev/null;
	echo "success."
endif
