fn main() {
    println!("cargo:rerun-if-changed=src/conntrack_events.c");
    cc::Build::new()
    .file("src/conntrack_events.c")
    .include("/home/vagrant/openwrt-sdk-21.02.0-rc4-x86-64_gcc-8.4.0_musl.Linux-x86_64/staging_dir/target-x86_64_musl/usr/include")
    .compile("conntrack_events");
    println!("cargo:rustc-link-search=/home/vagrant/openwrt-sdk-21.02.0-rc4-x86-64_gcc-8.4.0_musl.Linux-x86_64/staging_dir/target-x86_64_musl/usr/lib");

    println!("cargo:rustc-link-lib=netfilter_conntrack");
    println!("cargo:rustc-link-lib=nfnetlink");
    println!("cargo:rustc-link-lib=mnl");
}
