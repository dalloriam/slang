package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library", "rust_test")

rust_library(
    name = "assembler",
    srcs = glob([
        "src/**/.rs",
        "src/*.rs",
    ]),
    edition = "2018",
    deps = [
        "//cargo:nom",
        "//slang/instructor",
    ],
)

rust_test(
    name = "assembler_test",
    crate = ":assembler",
)
