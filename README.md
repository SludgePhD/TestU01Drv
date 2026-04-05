# TestU01Drv

`TestU01Drv` is a simple command-line application that can pipe the output of any external
random number generator into the [TestU01] randomness testing library.

This makes [TestU01], which normally requires linking against it as a library, easier to use.

[TestU01]: https://en.wikipedia.org/wiki/TestU01

## Usage

Test `/dev/urandom` with the `SmallCrush` and `LinComp` test batteries:

```shell
$ cat /dev/urandom | TestU01Drv --tests SmallCrush,LinComp
```

## Installation

### From GitHub releases

Every [GitHub release] contains pre-built statically linked binaries for Linux.

[GitHub release]: https://github.com/SludgePhD/TestU01Drv/releases

### From `crates.io`

Make sure a recent Rust toolchain is installed, then run:

```shell
$ cargo install TestU01Drv
```

### From Source

After checking out the repository (or downloading a source archive), run:

```shell
$ cargo install --path .
```

## License

TestU01 is published under a non-commercial license provided in [`TestU01-1.2.3/COPYING`].

The novel components of the TestU01Drv project are licensed under the same license.

[`TestU01-1.2.3/COPYING`]: ./TestU01-1.2.3/COPYING

## Modifications

The copy of TestU01 1.2.3 in this repository was lightly modified to fix compile errors and
slightly reduce verbosity.
All literate LaTeX files have also been pre-compiled into C headers, simplifying the build process.
