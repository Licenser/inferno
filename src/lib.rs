//! Inferno is a set of tools that let you to produce [flame graphs] from performance profiles of
//! your application. It's a port of parts Brendan Gregg's original [flamegraph toolkit] that aims
//! to improve the performance of the original flamegraph tools and provide programmatic access to
//! them to facilitate integration with _other_ tools (like [not-perf]).
//!
//! Inferno, like the original flame graph toolkit, consists of two "stages": stack collapsing and
//! plotting. In the original Perl implementations, these were represented by the `stackcollapse-*`
//! binaries and `flamegraph.pl` respectively. In Inferno, collapsing is available through the
//! [`collapse`] module and the `inferno-collapse-*` binaries, and plotting can be found in the
//! [`flamegraph`] module and the `inferno-flamegraph` binary.
//!
//! # Command-line use
//!
//! ## Collapsing stacks
//!
//! Most sampling profilers (as opposed to [tracing profilers]) work by repeatedly recording the
//! state of the [call stack]. The stack can be sampled based on a fixed sampling interval, based
//! on [hardware or software events], or some combination of the two. In the end, you get a series
//! of [stack traces], each of which represents a snapshot of where the program was at different
//! points in time.
//!
//! Given enough of these snapshots, you can get a pretty good idea of where your program is
//! spending its time by looking at which functions appear in many of the traces. To ease this
//! analysis, we want to "collapse" the stack traces so if a particular trace occurs more than
//! once, we instead just keep it _once_ along with a count of how many times we've seen it. This
//! is what the various collapsing tools do! You'll sometimes see the resulting tuples of stack +
//! count called a "folded stack trace".
//!
//! Since profiling tools produce stack traces in a myriad of different formats, and the flame
//! graph plotter expects input in a particular folded stack trace format, each profiler needs a
//! separate collapse implementation. While the original Perl implementation supports _lots_ of
//! profilers, Inferno currently only supports two: the widely used [`perf`] tool (specifically the
//! output from `perf script`) and [DTrace]. Support for xdebug is [hopefully coming soon], and
//! [`bpftrace`] should get [native support] before too long.
//!
//! Inferno supports profiles from applications written in any language, but we'll walk through an
//! example with a Rust program. To profile a Rust application, you would first set
//!
//! ```toml
//! [profile.release]
//! debug = true
//! ```
//!
//! in your `Cargo.toml` so that your profile will have useful function names and such included.
//! Then, compile with `--release`, and then run your favorite performance profiler:
//!
//! ### perf (Linux)
//!
//! ```console
//! # perf record --call-graph dwarf target/release/mybin
//! $ perf script | inferno-collapse-perf > stacks.folded
//! ```
//!
//! For more advanced uses, see Brendan Gregg's excellent [perf examples] page.
//!
//! ### DTrace (macOS)
//!
//! ```console
//! $ target/release/mybin &
//! $ pid=$!
//! # dtrace -x ustackframes=100 -n "profile-97 /pid == $pid/ { @[ustack()] = count(); } tick-60s { exit(0); }"  -o out.user_stacks
//! $ cat out.user_stacks | inferno-collapse-dtrace > stacks.folded
//! ```
//!
//! For more advanced uses, see also upstream FlameGraph's [DTrace examples].
//! You may also be interested in something like [NodeJS's ustack helper].
//!
//! ## Producing a flame graph
//!
//! Once you have a folded stack file, you're ready to produce the flame graph SVG image. To do so,
//! simply provide the folded stack file to `inferno-flamegraph`, and it will print the resulting
//! SVG. Following on from the example above:
//!
//! ```console
//! $ cat stacks.folded | inferno-flamegraph > profile.svg
//! ```
//!
//! And then open `profile.svg` in your viewer of choice.
//!
//! # Development
//!
//! This crate was initially developed through [a series of live coding sessions]. If you want to
//! contribute to the code, that may be a good way to learn why it's all designed the way it is!
//!
//!   [flame graphs]: http://www.brendangregg.com/flamegraphs.html
//!   [flamegraph toolkit]: https://github.com/brendangregg/FlameGraph
//!   [not-perf]: https://github.com/nokia/not-perf
//!   [tracing profilers]: https://danluu.com/perf-tracing/
//!   [call stack]: https://en.wikipedia.org/wiki/Call_stack
//!   [hardware or software events]: https://perf.wiki.kernel.org/index.php/Tutorial#Events
//!   [stack traces]: https://en.wikipedia.org/wiki/Stack_trace
//!   [`perf`]: https://perf.wiki.kernel.org/index.php/Main_Page
//!   [DTrace]: https://www.joyent.com/dtrace
//!   [hopefully coming soon]: https://twitter.com/DanielLockyer/status/1094605231155900416
//!   [native support]: https://github.com/jonhoo/inferno/issues/51#issuecomment-466732304
//!   [`bpftrace`]: https://github.com/iovisor/bpftrace
//!   [perf examples]: http://www.brendangregg.com/perf.html
//!   [DTrace examples]: http://www.brendangregg.com/FlameGraphs/cpuflamegraphs.html#DTrace
//!   [NodeJS's ustack helper]: http://dtrace.org/blogs/dap/2012/01/05/where-does-your-node-program-spend-its-time/
//!   [a series of live coding sessions]: https://www.youtube.com/watch?v=jTpK-bNZiA4&list=PLqbS7AVVErFimAvMW-kIJUwxpPvcPBCsz

#![deny(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
extern crate log;

/// Stack collapsing for various input formats.
///
/// See the [crate-level documentation] for details.
///
///   [crate-level documentation]: ../index.html
pub mod collapse;

/// Tools for producing flame graphs from folded stack traces.
///
/// See the [crate-level documentation] for details.
///
///   [crate-level documentation]: ../index.html
pub mod flamegraph;
