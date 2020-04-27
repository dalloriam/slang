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
        name = "raze__itoa__0_4_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itoa/itoa-0.4.5.crate",
        type = "tar.gz",
        strip_prefix = "itoa-0.4.5",

        build_file = Label("//cargo/remote:itoa-0.4.5.BUILD"),
    )

    _new_http_archive(
        name = "raze__proc_macro2__1_0_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/proc-macro2/proc-macro2-1.0.10.crate",
        type = "tar.gz",
        strip_prefix = "proc-macro2-1.0.10",

        build_file = Label("//cargo/remote:proc-macro2-1.0.10.BUILD"),
    )

    _new_http_archive(
        name = "raze__quote__1_0_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-1.0.3.crate",
        type = "tar.gz",
        strip_prefix = "quote-1.0.3",

        build_file = Label("//cargo/remote:quote-1.0.3.BUILD"),
    )

    _new_http_archive(
        name = "raze__ryu__1_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ryu/ryu-1.0.4.crate",
        type = "tar.gz",
        strip_prefix = "ryu-1.0.4",

        build_file = Label("//cargo/remote:ryu-1.0.4.BUILD"),
    )

    _new_http_archive(
        name = "raze__serde__1_0_106",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde/serde-1.0.106.crate",
        type = "tar.gz",
        strip_prefix = "serde-1.0.106",

        build_file = Label("//cargo/remote:serde-1.0.106.BUILD"),
    )

    _new_http_archive(
        name = "raze__serde_derive__1_0_106",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde_derive/serde_derive-1.0.106.crate",
        type = "tar.gz",
        strip_prefix = "serde_derive-1.0.106",

        build_file = Label("//cargo/remote:serde_derive-1.0.106.BUILD"),
    )

    _new_http_archive(
        name = "raze__serde_json__1_0_51",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde_json/serde_json-1.0.51.crate",
        type = "tar.gz",
        strip_prefix = "serde_json-1.0.51",

        build_file = Label("//cargo/remote:serde_json-1.0.51.BUILD"),
    )

    _new_http_archive(
        name = "raze__syn__1_0_18",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-1.0.18.crate",
        type = "tar.gz",
        strip_prefix = "syn-1.0.18",

        build_file = Label("//cargo/remote:syn-1.0.18.BUILD"),
    )

    _new_http_archive(
        name = "raze__unicode_xid__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-xid/unicode-xid-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "unicode-xid-0.2.0",

        build_file = Label("//cargo/remote:unicode-xid-0.2.0.BUILD"),
    )

