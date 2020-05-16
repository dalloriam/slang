load("@io_bazel_rules_rust//rust:rust.bzl", "rust_doc", "rust_library", "rust_test")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "assembler",
    srcs = glob([
        "src/**/.rs",
        "src/*.rs",
    ]),
    edition = "2018",
    deps = [
        "//cargo:nom",
        "//cargo:snafu",
        "//slang/instructor",
    ],
)

rust_library(
    name = "ft_crate",
    srcs = glob([
        "tests/**/*.rs",
        "tests/*.rs",
    ]),
    edition = "2018",
    deps = [
        ":assembler",
        "//slang/instructor",
    ],
)

rust_test(
    name = "test",
    crate = ":assembler",
)

rust_test(
    name = "functional_test",
    crate = ":ft_crate",
    data = glob(["tests/data/*"]),
)

rust_doc(
    name = "doc",
    dep = ":assembler",
)
