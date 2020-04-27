package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library", "rust_test")

rust_library(
    name = "slang",
    srcs = glob([
        "src/**/.rs",
        "src/*.rs",
    ]),
    deps = [],
)

rust_test(
    name = "slang_test",
    crate = ":slang",
)
