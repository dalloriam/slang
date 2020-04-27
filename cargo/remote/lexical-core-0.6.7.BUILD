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
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)

rust_binary(
    name = "lexical_core_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    edition = "2015",
    deps = [
        "@raze__rustc_version__0_2_3//:rustc_version",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    crate_features = [
      "arrayvec",
      "correct",
      "default",
      "ryu",
      "static_assertions",
      "std",
      "table",
    ],
    data = glob(["*"]),
    version = "0.6.7",
    visibility = ["//visibility:private"],
)

genrule(
    name = "lexical_core_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["lexical_core_out_dir_outputs.tar.gz"],
    tools = [
      ":lexical_core_build_script",
    ],
    tags = ["no-sandbox"],
    cmd = "mkdir -p $$(dirname $@)/lexical_core_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        # TODO(acmcarther): This needs to be revisited as part of the cross compilation story.
        #                   See also: https://github.com/google/cargo-raze/pull/54
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export CARGO_FEATURE_ARRAYVEC=1;"
        + " export CARGO_FEATURE_CORRECT=1;"
        + " export CARGO_FEATURE_DEFAULT=1;"
        + " export CARGO_FEATURE_RYU=1;"
        + " export CARGO_FEATURE_STATIC_ASSERTIONS=1;"
        + " export CARGO_FEATURE_STD=1;"
        + " export CARGO_FEATURE_TABLE=1;"
        + " export OUT_DIR=$$PWD/$$(dirname $@)/lexical_core_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :lexical_core_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)


rust_library(
    name = "lexical_core",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__arrayvec__0_4_12//:arrayvec",
        "@raze__bitflags__1_2_1//:bitflags",
        "@raze__cfg_if__0_1_9//:cfg_if",
        "@raze__ryu__1_0_4//:ryu",
        "@raze__static_assertions__0_3_4//:static_assertions",
    ],
    rustc_flags = [
        "--cap-lints=allow",
        "--cfg=limb_width_32",
        "--cfg=stable_i128",
    ],
    out_dir_tar = ":lexical_core_build_script_executor",
    version = "0.6.7",
    crate_features = [
        "arrayvec",
        "correct",
        "default",
        "ryu",
        "static_assertions",
        "std",
        "table",
    ],
)

