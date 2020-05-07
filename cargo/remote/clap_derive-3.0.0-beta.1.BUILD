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
    name = "clap_derive",
    crate_root = "src/lib.rs",
    crate_type = "proc-macro",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__heck__0_3_1//:heck",
        "@raze__proc_macro_error__0_4_12//:proc_macro_error",
        "@raze__proc_macro2__1_0_12//:proc_macro2",
        "@raze__quote__1_0_4//:quote",
        "@raze__syn__1_0_18//:syn",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "3.0.0-beta.1",
    crate_features = [
        "default",
    ],
)
