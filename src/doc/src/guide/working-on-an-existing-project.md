## Working on an Existing Crabgo Package

If you download an existing [package][def-package] that uses Crabgo, it’s
really easy to get going.

First, get the package from somewhere. In this example, we’ll use `regex`
cloned from its repository on GitHub:

```console
$ git clone https://github.com/rust-lang/regex.git
$ cd regex
```

To build, use `crabgo build`:

```console
$ crabgo build
   Compiling regex v1.5.0 (file:///path/to/package/regex)
```

This will fetch all of the dependencies and then build them, along with the
package.

[def-package]:  ../appendix/glossary.md#package  '"package" (glossary entry)'
