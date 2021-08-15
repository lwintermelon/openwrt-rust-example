export PATH=~/openwrt-sdk-21.02.0-rc4-x86-64_gcc-8.4.0_musl.Linux-x86_64/staging_dir/toolchain-x86_64_gcc-8.4.0_musl/bin/:$PATH
export STAGING_DIR=~/openwrt-sdk-21.02.0-rc4-x86-64_gcc-8.4.0_musl.Linux-x86_64/staging_dir/toolchain-x86_64_gcc-8.4.0_musl/bin/
CC=x86_64-openwrt-linux-musl-gcc cargo build --release