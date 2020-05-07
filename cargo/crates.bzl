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
        name = "raze__aho_corasick__0_7_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.7.10.crate",
        type = "tar.gz",
        strip_prefix = "aho-corasick-0.7.10",

        build_file = Label("//cargo/remote:aho-corasick-0.7.10.BUILD"),
    )

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
        name = "raze__atty__0_2_14",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atty/atty-0.2.14.crate",
        type = "tar.gz",
        strip_prefix = "atty-0.2.14",

        build_file = Label("//cargo/remote:atty-0.2.14.BUILD"),
    )

    _new_http_archive(
        name = "raze__autocfg__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/autocfg/autocfg-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "autocfg-1.0.0",

        build_file = Label("//cargo/remote:autocfg-1.0.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__bitflags__1_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-1.2.1.crate",
        type = "tar.gz",
        strip_prefix = "bitflags-1.2.1",

        build_file = Label("//cargo/remote:bitflags-1.2.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__byteorder__1_3_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/byteorder/byteorder-1.3.4.crate",
        type = "tar.gz",
        strip_prefix = "byteorder-1.3.4",

        build_file = Label("//cargo/remote:byteorder-1.3.4.BUILD"),
    )

    _new_http_archive(
        name = "raze__cfg_if__0_1_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.9.crate",
        type = "tar.gz",
        strip_prefix = "cfg-if-0.1.9",

        build_file = Label("//cargo/remote:cfg-if-0.1.9.BUILD"),
    )

    _new_http_archive(
        name = "raze__clap__3_0_0_beta_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clap/clap-3.0.0-beta.1.crate",
        type = "tar.gz",
        strip_prefix = "clap-3.0.0-beta.1",

        build_file = Label("//cargo/remote:clap-3.0.0-beta.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__clap_derive__3_0_0_beta_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clap_derive/clap_derive-3.0.0-beta.1.crate",
        type = "tar.gz",
        strip_prefix = "clap_derive-3.0.0-beta.1",

        build_file = Label("//cargo/remote:clap_derive-3.0.0-beta.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__env_logger__0_7_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/env_logger/env_logger-0.7.1.crate",
        type = "tar.gz",
        strip_prefix = "env_logger-0.7.1",

        build_file = Label("//cargo/remote:env_logger-0.7.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__heck__0_3_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/heck/heck-0.3.1.crate",
        type = "tar.gz",
        strip_prefix = "heck-0.3.1",

        build_file = Label("//cargo/remote:heck-0.3.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__hermit_abi__0_1_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hermit-abi/hermit-abi-0.1.12.crate",
        type = "tar.gz",
        strip_prefix = "hermit-abi-0.1.12",

        build_file = Label("//cargo/remote:hermit-abi-0.1.12.BUILD"),
    )

    _new_http_archive(
        name = "raze__humantime__1_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/humantime/humantime-1.3.0.crate",
        type = "tar.gz",
        strip_prefix = "humantime-1.3.0",

        build_file = Label("//cargo/remote:humantime-1.3.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__indexmap__1_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/indexmap/indexmap-1.3.2.crate",
        type = "tar.gz",
        strip_prefix = "indexmap-1.3.2",

        build_file = Label("//cargo/remote:indexmap-1.3.2.BUILD"),
    )

    _new_http_archive(
        name = "raze__lazy_static__1_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-1.4.0.crate",
        type = "tar.gz",
        strip_prefix = "lazy_static-1.4.0",

        build_file = Label("//cargo/remote:lazy_static-1.4.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__lexical_core__0_6_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lexical-core/lexical-core-0.6.7.crate",
        type = "tar.gz",
        strip_prefix = "lexical-core-0.6.7",

        build_file = Label("//cargo/remote:lexical-core-0.6.7.BUILD"),
    )

    _new_http_archive(
        name = "raze__libc__0_2_69",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.69.crate",
        type = "tar.gz",
        strip_prefix = "libc-0.2.69",

        build_file = Label("//cargo/remote:libc-0.2.69.BUILD"),
    )

    _new_http_archive(
        name = "raze__log__0_4_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.4.8.crate",
        type = "tar.gz",
        strip_prefix = "log-0.4.8",

        build_file = Label("//cargo/remote:log-0.4.8.BUILD"),
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
        name = "raze__os_str_bytes__2_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/os_str_bytes/os_str_bytes-2.3.0.crate",
        type = "tar.gz",
        strip_prefix = "os_str_bytes-2.3.0",

        build_file = Label("//cargo/remote:os_str_bytes-2.3.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__proc_macro_error__0_4_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/proc-macro-error/proc-macro-error-0.4.12.crate",
        type = "tar.gz",
        strip_prefix = "proc-macro-error-0.4.12",

        build_file = Label("//cargo/remote:proc-macro-error-0.4.12.BUILD"),
    )

    _new_http_archive(
        name = "raze__proc_macro_error_attr__0_4_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/proc-macro-error-attr/proc-macro-error-attr-0.4.12.crate",
        type = "tar.gz",
        strip_prefix = "proc-macro-error-attr-0.4.12",

        build_file = Label("//cargo/remote:proc-macro-error-attr-0.4.12.BUILD"),
    )

    _new_http_archive(
        name = "raze__proc_macro2__1_0_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/proc-macro2/proc-macro2-1.0.12.crate",
        type = "tar.gz",
        strip_prefix = "proc-macro2-1.0.12",

        build_file = Label("//cargo/remote:proc-macro2-1.0.12.BUILD"),
    )

    _new_http_archive(
        name = "raze__quick_error__1_2_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quick-error/quick-error-1.2.3.crate",
        type = "tar.gz",
        strip_prefix = "quick-error-1.2.3",

        build_file = Label("//cargo/remote:quick-error-1.2.3.BUILD"),
    )

    _new_http_archive(
        name = "raze__quote__1_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-1.0.4.crate",
        type = "tar.gz",
        strip_prefix = "quote-1.0.4",

        build_file = Label("//cargo/remote:quote-1.0.4.BUILD"),
    )

    _new_http_archive(
        name = "raze__regex__1_3_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-1.3.7.crate",
        type = "tar.gz",
        strip_prefix = "regex-1.3.7",

        build_file = Label("//cargo/remote:regex-1.3.7.BUILD"),
    )

    _new_http_archive(
        name = "raze__regex_syntax__0_6_17",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.6.17.crate",
        type = "tar.gz",
        strip_prefix = "regex-syntax-0.6.17",

        build_file = Label("//cargo/remote:regex-syntax-0.6.17.BUILD"),
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
        name = "raze__strsim__0_10_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/strsim/strsim-0.10.0.crate",
        type = "tar.gz",
        strip_prefix = "strsim-0.10.0",

        build_file = Label("//cargo/remote:strsim-0.10.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__syn__1_0_18",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-1.0.18.crate",
        type = "tar.gz",
        strip_prefix = "syn-1.0.18",

        build_file = Label("//cargo/remote:syn-1.0.18.BUILD"),
    )

    _new_http_archive(
        name = "raze__syn_mid__0_5_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn-mid/syn-mid-0.5.0.crate",
        type = "tar.gz",
        strip_prefix = "syn-mid-0.5.0",

        build_file = Label("//cargo/remote:syn-mid-0.5.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__termcolor__1_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termcolor/termcolor-1.1.0.crate",
        type = "tar.gz",
        strip_prefix = "termcolor-1.1.0",

        build_file = Label("//cargo/remote:termcolor-1.1.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__textwrap__0_11_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/textwrap/textwrap-0.11.0.crate",
        type = "tar.gz",
        strip_prefix = "textwrap-0.11.0",

        build_file = Label("//cargo/remote:textwrap-0.11.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__thread_local__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-1.0.1.crate",
        type = "tar.gz",
        strip_prefix = "thread_local-1.0.1",

        build_file = Label("//cargo/remote:thread_local-1.0.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__unicode_segmentation__1_6_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-segmentation/unicode-segmentation-1.6.0.crate",
        type = "tar.gz",
        strip_prefix = "unicode-segmentation-1.6.0",

        build_file = Label("//cargo/remote:unicode-segmentation-1.6.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__unicode_width__0_1_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-width/unicode-width-0.1.7.crate",
        type = "tar.gz",
        strip_prefix = "unicode-width-0.1.7",

        build_file = Label("//cargo/remote:unicode-width-0.1.7.BUILD"),
    )

    _new_http_archive(
        name = "raze__unicode_xid__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-xid/unicode-xid-0.2.0.crate",
        type = "tar.gz",
        strip_prefix = "unicode-xid-0.2.0",

        build_file = Label("//cargo/remote:unicode-xid-0.2.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__vec_map__0_8_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/vec_map/vec_map-0.8.1.crate",
        type = "tar.gz",
        strip_prefix = "vec_map-0.8.1",

        build_file = Label("//cargo/remote:vec_map-0.8.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__version_check__0_9_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/version_check/version_check-0.9.1.crate",
        type = "tar.gz",
        strip_prefix = "version_check-0.9.1",

        build_file = Label("//cargo/remote:version_check-0.9.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi__0_3_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.8.crate",
        type = "tar.gz",
        strip_prefix = "winapi-0.3.8",

        build_file = Label("//cargo/remote:winapi-0.3.8.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",

        build_file = Label("//cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi_util__0_1_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-util/winapi-util-0.1.5.crate",
        type = "tar.gz",
        strip_prefix = "winapi-util-0.1.5",

        build_file = Label("//cargo/remote:winapi-util-0.1.5.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",

        build_file = Label("//cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD"),
    )

