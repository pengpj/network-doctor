all: build

# 检查是不是 arm64 或 aarch64，如果是则编译 arm64 版本，否则编译 x86_64 版本
build:
	@echo "Building..."
	@arch=$$(uname -m); \
	echo "host uname -m: " $$arch; \
	if [ "$$arch" = "aarch64" ] || [ "$$arch" = "arm64" ]; then \
		make build-arm64; \
	else \
		make build-x86_64; \
	fi

build-arm64:
	@echo "Building for arm64..."
	@env TERM=xterm-256color; \
	cargo build --release --color=always 2>&1

build-x86_64:
	@echo "Building for x86_64..."
	@env TERM=xterm-256color; \
	cargo build --release --target x86_64-unknown-linux-musl --color=always 2>&1
	@if [ -d "/data/WebRoot/download/pz" ]; then \
	    echo "Copy to /data/WebRoot/download/pz"; \
		cp target/x86_64-unknown-linux-musl/release/doctor /data/WebRoot/download/pz/; \
	else \
		echo "No /data/WebRoot/download/pz"; \
	fi