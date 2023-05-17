## Continuous Integration

### Travis CI

To test your [package][def-package] on Travis CI, here is a sample
`.travis.yml` file:

```yaml
language: rust
rust:
  - stable
  - beta
  - nightly
matrix:
  allow_failures:
    - rust: nightly
```

This will test all three release channels, but any breakage in nightly
will not fail your overall build. Please see the [Travis CI Rust
documentation](https://docs.travis-ci.com/user/languages/rust/) for more
information.

### GitHub Actions

To test your package on GitHub Actions, here is a sample `.github/workflows/ci.yml` file:

```yaml
name: Crabgo Build & Test

on:
  push:
  pull_request:

env: 
  CRABGO_TERM_COLOR: always

jobs:
  build_and_test:
    name: Rust project - latest
    runs-on: ubuntu-latest
    strategy:
      matrix:
        toolchain:
          - stable
          - beta
          - nightly
    steps:
      - uses: actions/checkout@v3
      - run: rustup update ${{ matrix.toolchain }} && rustup default ${{ matrix.toolchain }}
      - run: crabgo build --verbose
      - run: crabgo test --verbose
  
```

This will test all three release channels (note a failure in any toolchain version will fail the entire job). You can also click `"Actions" > "new workflow"` in the GitHub UI and select Rust to add the [default configuration](https://github.com/actions/starter-workflows/blob/main/ci/rust.yml) to your repo. See [GitHub Actions documentation](https://docs.github.com/en/actions) for more information.

### GitLab CI

To test your package on GitLab CI, here is a sample `.gitlab-ci.yml` file:

```yaml
stages:
  - build

rust-latest:
  stage: build
  image: rust:latest
  script:
    - crabgo build --verbose
    - crabgo test --verbose

rust-nightly:
  stage: build
  image: rustlang/rust:nightly
  script:
    - crabgo build --verbose
    - crabgo test --verbose
  allow_failure: true
```

This will test on the stable channel and nightly channel, but any
breakage in nightly will not fail your overall build. Please see the
[GitLab CI documentation](https://docs.gitlab.com/ce/ci/yaml/index.html) for more
information.

### builds.sr.ht

To test your package on sr.ht, here is a sample `.build.yml` file.
Be sure to change `<your repo>` and `<your project>` to the repo to clone and
the directory where it was cloned.

```yaml
image: archlinux
packages:
  - rustup
sources:
  - <your repo>
tasks:
  - setup: |
      rustup toolchain install nightly stable
      cd <your project>/
      rustup run stable crabgo fetch
  - stable: |
      rustup default stable
      cd <your project>/
      crabgo build --verbose
      crabgo test --verbose
  - nightly: |
      rustup default nightly
      cd <your project>/
      crabgo build --verbose ||:
      crabgo test --verbose  ||:
  - docs: |
      cd <your project>/
      rustup run stable crabgo doc --no-deps
      rustup run nightly crabgo doc --no-deps ||:
```

This will test and build documentation on the stable channel and nightly
channel, but any breakage in nightly will not fail your overall build. Please
see the [builds.sr.ht documentation](https://man.sr.ht/builds.sr.ht/) for more
information.

[def-package]:  ../appendix/glossary.md#package  '"package" (glossary entry)'
