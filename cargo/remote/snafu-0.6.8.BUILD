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


# Unsupported target "backtrace" with type "test" omitted
# Unsupported target "backtrace-optional" with type "test" omitted
# Unsupported target "backtrace-optional-enabled" with type "test" omitted
# Unsupported target "backtrace_attributes" with type "test" omitted
# Unsupported target "basic" with type "test" omitted
# Unsupported target "boxed_error_trait_object" with type "test" omitted
# Unsupported target "build-leaf-error" with type "test" omitted
# Unsupported target "default_error_display" with type "test" omitted
# Unsupported target "doc_comment" with type "test" omitted
# Unsupported target "generics" with type "test" omitted
# Unsupported target "generics_with_default" with type "test" omitted
# Unsupported target "mapping_result_without_try_operator" with type "test" omitted
# Unsupported target "multiple_attributes" with type "test" omitted
# Unsupported target "name-conflicts" with type "test" omitted
# Unsupported target "no_context" with type "test" omitted
# Unsupported target "opaque" with type "test" omitted
# Unsupported target "options" with type "test" omitted
# Unsupported target "recursive_error" with type "test" omitted
# Unsupported target "send_between_threads" with type "test" omitted
# Unsupported target "single_use_lifetimes_lint" with type "test" omitted

rust_library(
    name = "snafu",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__doc_comment__0_3_3//:doc_comment",
        "@raze__snafu_derive__0_6_8//:snafu_derive",
    ],
    rustc_flags = [
        "--cap-lints=allow",
        "--cfg=use_proc_macro",
    ],
    data = glob(["src/**", "README.md"]),
    version = "0.6.8",
    crate_features = [
        "default",
        "guide",
        "std",
    ],
)

# Unsupported target "source_attributes" with type "test" omitted
# Unsupported target "visibility" with type "test" omitted
