#[macro_use]
extern crate pretty_assertions;

extern crate inferno;

mod collapse_common;

use collapse_common::*;
use inferno::collapse::dtrace::{Folder, Options};
use std::io;

fn test_collapse_dtrace(test_file: &str, expected_file: &str, options: Options) -> io::Result<()> {
    test_collapse(Folder::from(options), test_file, expected_file)
}

#[test]
fn collapse_dtrace_compare_to_upstream() {
    let test_file = "./flamegraph/example-dtrace-stacks.txt";
    let result_file = "./tests/data/collapse-dtrace/results/dtrace-example.txt";
    test_collapse_dtrace(test_file, result_file, Options::default()).unwrap()
}

#[test]
fn collapse_dtrace_compare_to_upstream_with_offsets() {
    let test_file = "./flamegraph/example-dtrace-stacks.txt";
    let result_file = "./tests/data/collapse-dtrace/results/dtrace-example-offsets.txt";
    test_collapse_dtrace(
        test_file,
        result_file,
        Options {
            includeoffset: true,
        },
    )
    .unwrap()
}

#[test]
fn collapse_dtrace_compare_to_upstream_java() {
    let test_file = "./tests/data/collapse-dtrace/java.txt";
    let result_file = "./tests/data/collapse-dtrace/results/java.txt";
    test_collapse_dtrace(test_file, result_file, Options::default()).unwrap()
}

#[test]
fn collapse_dtrace_hex_addresses() {
    let test_file = "./tests/data/collapse-dtrace/hex-addresses.txt";
    let result_file = "./tests/data/collapse-dtrace/results/hex-addresses.txt";
    test_collapse_dtrace(test_file, result_file, Options::default()).unwrap()
}

#[test]
fn collapse_dtrace_compare_to_flamegraph_bug() {
    // There is a bug in flamegraph that causes the following stack to render
    // badly. We fix this but keep the test around to point out this breakage
    // of bug compatibility.
    //
    // https://github.com/brendangregg/FlameGraph/issues/202
    let test_file = "./tests/data/collapse-dtrace/flamegraph-bug.txt";
    let result_file = "./tests/data/collapse-dtrace/results/flamegraph-bug.txt";
    test_collapse_dtrace(
        test_file,
        result_file,
        Options {
            includeoffset: true,
        },
    )
    .unwrap()
}
