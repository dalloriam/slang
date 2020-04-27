"""
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")

def _new_http_archive(name, **kwargs):
    if not native.existing_rule(name):
        http_archive(name=name, **kwargs)

def _new_git_repository(name, **kwargs):
    if not native.existing_rule(name):
        new_git_repository(name=name, **kwargs)

def raze_fetch_remote_crates():

    _new_http_archive(
        name = "raze__anyhow__1_0_28",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/anyhow/anyhow-1.0.28.crate",
        type = "tar.gz",
        strip_prefix = "anyhow-1.0.28",

        build_file = Label("//cargo/remote:anyhow-1.0.28.BUILD"),
    )

    _new_http_archive(
        name = "raze__arrayvec__0_4_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/arrayvec/arrayvec-0.4.12.crate",
        type = "tar.gz",
        strip_prefix = "arrayvec-0.4.12",

        build_file = Label("//cargo/remote:arrayvec-0.4.12.BUILD"),
    )

    _new_http_archive(
        name = "raze__bitflags__1_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-1.2.1.crate",
        type = "tar.gz",
        strip_prefix = "bitflags-1.2.1",

        build_file = Label("//cargo/remote:bitflags-1.2.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__cfg_if__0_1_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.9.crate",
        type = "tar.gz",
        strip_prefix = "cfg-if-0.1.9",

        build_file = Label("//cargo/remote:cfg-if-0.1.9.BUILD"),
    )

    _new_http_archive(
        name = "raze__lexical_core__0_6_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lexical-core/lexical-core-0.6.7.crate",
        type = "tar.gz",
        strip_prefix = "lexical-core-0.6.7",

        build_file = Label("//cargo/remote:lexical-core-0.6.7.BUILD"),
    )

    _new_http_archive(
        name = "raze__memchr__2_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-2.3.3.crate",
        type = "tar.gz",
        strip_prefix = "memchr-2.3.3",

        build_file = Label("//cargo/remote:memchr-2.3.3.BUILD"),
    )

    _new_http_archive(
        name = "raze__nodrop__0_1_14",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/nodrop/nodrop-0.1.14.crate",
        type = "tar.gz",
        strip_prefix = "nodrop-0.1.14",

        build_file = Label("//cargo/remote:nodrop-0.1.14.BUILD"),
    )

    _new_http_archive(
        name = "raze__nom__5_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/nom/nom-5.1.1.crate",
        type = "tar.gz",
        strip_prefix = "nom-5.1.1",

        build_file = Label("//cargo/remote:nom-5.1.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__rustc_version__0_2_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rustc_version/rustc_version-0.2.3.crate",
        type = "tar.gz",
        strip_prefix = "rustc_version-0.2.3",

        build_file = Label("//cargo/remote:rustc_version-0.2.3.BUILD"),
    )

    _new_http_archive(
        name = "raze__ryu__1_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ryu/ryu-1.0.4.crate",
        type = "tar.gz",
        strip_prefix = "ryu-1.0.4",

        build_file = Label("//cargo/remote:ryu-1.0.4.BUILD"),
    )

    _new_http_archive(
        name = "raze__semver__0_9_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/semver/semver-0.9.0.crate",
        type = "tar.gz",
        strip_prefix = "semver-0.9.0",

        build_file = Label("//cargo/remote:semver-0.9.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__semver_parser__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/semver-parser/semver-parser-0.7.0.crate",
        type = "tar.gz",
        strip_prefix = "semver-parser-0.7.0",

        build_file = Label("//cargo/remote:semver-parser-0.7.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__static_assertions__0_3_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/static_assertions/static_assertions-0.3.4.crate",
        type = "tar.gz",
        strip_prefix = "static_assertions-0.3.4",

        build_file = Label("//cargo/remote:static_assertions-0.3.4.BUILD"),
    )

    _new_http_archive(
        name = "raze__version_check__0_9_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/version_check/version_check-0.9.1.crate",
        type = "tar.gz",
        strip_prefix = "version_check-0.9.1",

        build_file = Label("//cargo/remote:version_check-0.9.1.BUILD"),
    )

