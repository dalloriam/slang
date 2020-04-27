load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "repl",
    srcs = glob([
        "src/**/.rs",
        "src/*.rs",
    ]),
    edition = "2018",
    deps = [
        "//cargo:anyhow",
        "//slang",
    ],
)
