# TestU01Drv

`TestU01Drv` is a simple command-line application that can pipe the output of any external
random number generator into the [TestU01] randomness testing library.

This makes [TestU01], which normally requires linking against it as a library, easier to use.

[TestU01]: https://en.wikipedia.org/wiki/TestU01

## Installation

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
