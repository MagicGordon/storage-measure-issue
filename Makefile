build: lint storage_measure_issue

lint:
	@cargo fmt --all
	@cargo clippy --fix --allow-dirty --allow-staged

storage_measure_issue:
	$(call local_build_wasm,storage_measure_issue)

define local_build_wasm
	$(eval WASM_NAME := $(1))

	@mkdir -p res
	@rustup target add wasm32-unknown-unknown
	@cargo near build --no-docker
	@cp target/near/$(WASM_NAME).wasm ./res/$(WASM_NAME).wasm
endef