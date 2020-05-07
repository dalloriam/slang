load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "slang-cli",
    srcs = glob([
        "src/**/.rs",
        "src/*.rs",
    ]),
    edition = "2018",
    deps = [
        "//cargo:anyhow",
        "//cargo:clap",
        "//slang/assembler",
        "//slang/vm",
    ],
)
