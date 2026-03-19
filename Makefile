CARGO := cargo
GENIE := genie
PROJECT_NAME := j_run

.PHONY: all
all: build

.PHONY: setup
setup:
	$(GENIE) install

.PHONY: build
build:
	$(CARGO) build

.PHONY: release
release:
	$(CARGO) build --release

.PHONY: run
run:
	$(CARGO) run

.PHONY: genie-run
genie-run:
	$(GENIE) run -- $(CARGO) run

.PHONY: fmt
fmt:
	$(CARGO) fmt

.PHONY: lint
lint:
	$(CARGO) clippy -- -D warnings

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: push
push:
	./push.sh
