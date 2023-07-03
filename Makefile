ROOT_DIR := $(shell pwd)
BUILD_DIR := target/arm-unknown-linux-gnueabihf/release
DIST_DIR := dist
RETROARCH := third-party/RetroArch
TOOLCHAIN := mholdg16/miyoomini-toolchain:latest

all: static build package-build package-retroarch

simulator-env:
	mkdir -p assets/simulator
	rsync -ar assets/root/.allium assets/simulator/

simulator-launcher: simulator-env
	RUST_LOG=trace RUST_BACKTRACE=1 ALLIUM_DATABASE=assets/simulator/allium.db ALLIUM_BASE_DIR=assets/simulator/.allium ALLIUM_SD_ROOT=assets/simulator cargo run --bin allium-launcher --features=simulator

simulator-menu: simulator-env
	RUST_LOG=trace RUST_BACKTRACE=1 ALLIUM_DATABASE=assets/simulator/allium.db ALLIUM_BASE_DIR=assets/simulator/.allium ALLIUM_SD_ROOT=assets/simulator cargo run --bin allium-menu --features=simulator

simulator: simulator-env
	RUST_LOG=trace RUST_BACKTRACE=1 ALLIUM_DATABASE=assets/simulator/allium.db ALLIUM_BASE_DIR=assets/simulator/.allium ALLIUM_SD_ROOT=assets/simulator cargo run --bin $(bin) --features=simulator

clean:
	rm -r $(DIST_DIR)

static:
	mkdir -p $(DIST_DIR)
	rsync -a --exclude='.gitkeep' assets/root/. $(DIST_DIR)

third-party/my283:
	wget -O third-party/my283.tar.xz https://github.com/shauninman/miyoomini-toolchain-buildroot/raw/main/support/my283.tar.xz
	cd third-party/ && tar xf my283.tar.xz
	rm third-party/my283.tar.xz

build: third-party/my283
	cross build --release --features=miyoo --bin=alliumd --bin=allium-launcher --bin=allium-menu --bin=activity-tracker

build-with-console: third-party/my283
	cross build --release --features=miyoo,console --bin=alliumd --bin=allium-launcher --bin=allium-menu --bin=activity-tracker

package-build:
	mkdir -p $(DIST_DIR)/.allium/bin
	rsync -a $(BUILD_DIR)/alliumd $(DIST_DIR)/.allium/bin/
	rsync -a $(BUILD_DIR)/allium-launcher $(DIST_DIR)/.allium/bin/
	rsync -a $(BUILD_DIR)/allium-menu $(DIST_DIR)/.allium/bin/
	rsync -a $(BUILD_DIR)/activity-tracker "$(DIST_DIR)/Apps/Activity Tracker.pak/"

retroarch: $(RETROARCH)/retroarch_miyoo283 $(RETROARCH)/retroarch_miyoo354

package-retroarch: retroarch
	rsync -a $(RETROARCH)/retroarch_miyoo354 $(DIST_DIR)/RetroArch/
	rsync -a $(RETROARCH)/retroarch_miyoo283 $(DIST_DIR)/RetroArch/

$(RETROARCH)/retroarch_miyoo354:
	docker run --rm -v /$(ROOT_DIR)/third-party:/root/workspace $(TOOLCHAIN) bash -c "source /root/.bashrc; cd RetroArch; make clean all ADD_NETWORKING=1 PACKAGE_NAME=retroarch_miyoo354"

$(RETROARCH)/retroarch_miyoo283:
	docker run --rm -v /$(ROOT_DIR)/third-party:/root/workspace $(TOOLCHAIN) bash -c "source /root/.bashrc; cd RetroArch; make clean all PACKAGE_NAME=retroarch_miyoo283"

lint:
	cargo fmt
	cargo clippy --fix --allow-dirty --allow-staged --all-targets

bump-version: lint
	sed -i "s/^version = \".*\"/version = \"$(version)\"/" allium-launcher/Cargo.toml
	sed -i "s/^version = \".*\"/version = \"$(version)\"/" allium-menu/Cargo.toml
	sed -i "s/^version = \".*\"/version = \"$(version)\"/" alliumd/Cargo.toml
	sed -i "s/^version = \".*\"/version = \"$(version)\"/" activity-tracker/Cargo.toml
	sed -i "s/^version = \".*\"/version = \"$(version)\"/" common/Cargo.toml
	cargo check
	git add allium-launcher/Cargo.toml
	git add allium-menu/Cargo.toml
	git add alliumd/Cargo.toml
	git add activity-tracker/Cargo.toml
	git add common/Cargo.toml
	git add Cargo.lock
	git commit -m "chore: bump version to v$(version)"
	git tag "v$(version)" -a
