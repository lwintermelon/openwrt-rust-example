# OpenWrt Rust Boilerplate

this project links to OpenWrt's `libnetfilter_conntrack` C lib as an example, and produce a very small binary.
## about the binary size

Use cargo unstable features to reduce the binary size,  the final binary is ~14 kilobytes on x86-64, it's comparable to C, and suitable for embedded systems.