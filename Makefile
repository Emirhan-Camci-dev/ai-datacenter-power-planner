.PHONY: build-core test-core build-pro test-pro

# Community Edition
build-core:
	cd aigrid_planner_core && cargo build --release

test-core:
	cd aigrid_planner_core && cargo test

# Enterprise Edition
build-pro:
	cd aigrid_planner_pro && cargo build --release

test-pro:
	cd aigrid_planner_pro && cargo test

# Python SDK Compilation (maturin/pyo3)
build-python-sdk:
	cd aigrid_planner_pro && maturin build --release
