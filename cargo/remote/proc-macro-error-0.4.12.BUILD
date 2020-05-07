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

rust_binary(
    name = "proc_macro_error_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    edition = "2018",
    deps = [
        "@raze__version_check__0_9_1//:version_check",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    crate_features = [
    ],
    data = glob(["*"]),
    version = "0.4.12",
    visibility = ["//visibility:private"],
)

genrule(
    name = "proc_macro_error_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["proc_macro_error_out_dir_outputs.tar.gz"],
    tools = [
      ":proc_macro_error_build_script",
    ],
    tags = ["no-sandbox"],
    cmd = "mkdir -p $$(dirname $@)/proc_macro_error_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        # TODO(acmcarther): This needs to be revisited as part of the cross compilation story.
        #                   See also: https://github.com/google/cargo-raze/pull/54
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export OUT_DIR=$$PWD/$$(dirname $@)/proc_macro_error_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :proc_macro_error_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)

# Unsupported target "macro-errors" with type "test" omitted
# Unsupported target "ok" with type "test" omitted

rust_library(
    name = "proc_macro_error",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__proc_macro_error_attr__0_4_12//:proc_macro_error_attr",
        "@raze__proc_macro2__1_0_12//:proc_macro2",
        "@raze__quote__1_0_4//:quote",
        "@raze__syn__1_0_18//:syn",
    ],
    rustc_flags = [
        "--cap-lints=allow",
        "--cfg=use_fallback",
    ],
    out_dir_tar = ":proc_macro_error_build_script_executor",
    version = "0.4.12",
    crate_features = [
    ],
)

