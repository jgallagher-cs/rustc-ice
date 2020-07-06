`foo-linux` contains files built on a Linux (CentOS 6) system, via:

```
# in foo-linux, on a Linux system (or in a Linux container)
gcc -c ../foo.c -o foo.o
ar cr libfoo.a foo.o
```

`foo-mac` contains files built on Mac OS X Catalina, but starting from the Linux archive (which is super weird and probably shouldn't work):

```
# in foo-mac, on OS X Catalina
cp ../foo-linux/libfoo.a .
clang -c ../foo.c -o foo.o
ar cr libfoo.a foo.o
```

Now trying to compile the Rust crate which tries to link `foo-mac/libfoo.a` ICEs:

```
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_codegen_llvm/back/archive.rs:274:33
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0 (49cae5576 2020-06-01) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental -C link-args=-fopenmp --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: could not compile `rustc-ice`.

To learn more, run the command again with --verbose.
```
