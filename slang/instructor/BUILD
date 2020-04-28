package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_doc", "rust_library", "rust_test")

rust_library(
    name = "instructor",
    srcs = glob([
        "src/**/.rs",
        "src/*.rs",
    ]),
    edition = "2018",
    deps = [],
)

rust_test(
    name = "test",
    crate = ":instructor",
)

rust_doc(
    name = "doc",
    dep = ":instructor",
)
