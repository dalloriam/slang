"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "restricted", # "MIT OR Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "clap",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__atty__0_2_14//:atty",
        "@raze__bitflags__1_2_1//:bitflags",
        "@raze__clap_derive__3_0_0_beta_1//:clap_derive",
        "@raze__indexmap__1_3_2//:indexmap",
        "@raze__lazy_static__1_4_0//:lazy_static",
        "@raze__os_str_bytes__2_3_0//:os_str_bytes",
        "@raze__strsim__0_10_0//:strsim",
        "@raze__termcolor__1_1_0//:termcolor",
        "@raze__textwrap__0_11_0//:textwrap",
        "@raze__unicode_width__0_1_7//:unicode_width",
        "@raze__vec_map__0_8_1//:vec_map",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "3.0.0-beta.1",
    crate_features = [
        "atty",
        "cargo",
        "clap_derive",
        "color",
        "default",
        "derive",
        "lazy_static",
        "std",
        "strsim",
        "suggestions",
        "termcolor",
    ],
)

