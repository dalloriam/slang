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
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)

rust_binary(
    name = "nom_build_script",
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
      "alloc",
      "default",
      "lexical",
      "lexical-core",
      "std",
    ],
    data = glob(["*"]),
    version = "5.1.1",
    visibility = ["//visibility:private"],
)

genrule(
    name = "nom_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["nom_out_dir_outputs.tar.gz"],
    tools = [
      ":nom_build_script",
    ],
    tags = ["no-sandbox"],
    cmd = "mkdir -p $$(dirname $@)/nom_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        # TODO(acmcarther): This needs to be revisited as part of the cross compilation story.
        #                   See also: https://github.com/google/cargo-raze/pull/54
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export CARGO_FEATURE_ALLOC=1;"
        + " export CARGO_FEATURE_DEFAULT=1;"
        + " export CARGO_FEATURE_LEXICAL=1;"
        + " export CARGO_FEATURE_LEXICAL_CORE=1;"
        + " export CARGO_FEATURE_STD=1;"
        + " export OUT_DIR=$$PWD/$$(dirname $@)/nom_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :nom_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)

# Unsupported target "arithmetic" with type "bench" omitted
# Unsupported target "arithmetic" with type "test" omitted
# Unsupported target "arithmetic_ast" with type "test" omitted
# Unsupported target "blockbuf-arithmetic" with type "test" omitted
# Unsupported target "css" with type "test" omitted
# Unsupported target "custom_errors" with type "test" omitted
# Unsupported target "escaped" with type "test" omitted
# Unsupported target "float" with type "test" omitted
# Unsupported target "http" with type "bench" omitted
# Unsupported target "inference" with type "test" omitted
# Unsupported target "ini" with type "bench" omitted
# Unsupported target "ini" with type "test" omitted
# Unsupported target "ini_complete" with type "bench" omitted
# Unsupported target "ini_str" with type "bench" omitted
# Unsupported target "ini_str" with type "test" omitted
# Unsupported target "issues" with type "test" omitted
# Unsupported target "json" with type "bench" omitted
# Unsupported target "json" with type "example" omitted
# Unsupported target "json" with type "test" omitted
# Unsupported target "mp4" with type "test" omitted
# Unsupported target "multiline" with type "test" omitted
# Unsupported target "named_args" with type "test" omitted

rust_library(
    name = "nom",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__lexical_core__0_6_7//:lexical_core",
        "@raze__memchr__2_3_3//:memchr",
    ],
    rustc_flags = [
        "--cap-lints=allow",
        "--cfg=stable_i128",
    ],
    out_dir_tar = ":nom_build_script_executor",
    version = "5.1.1",
    crate_features = [
        "alloc",
        "default",
        "lexical",
        "lexical-core",
        "std",
    ],
)

# Unsupported target "overflow" with type "test" omitted
# Unsupported target "reborrow_fold" with type "test" omitted
# Unsupported target "s_expression" with type "example" omitted
# Unsupported target "string" with type "example" omitted
# Unsupported target "test1" with type "test" omitted
