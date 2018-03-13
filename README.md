# [Codecov][1] Rust Example

**As of July 2, 2016, there is [no option to make rustdoc generate a runnable test executable][7]. That means that documentation tests will not show in your coverage data. If you discover a way to run the doctest executable with kcov, please open an [Issue][8] and we will add that to these instructions.**

## Guide
### Travis Setup

Add to your `.travis.yml` file.
```yml
language: rust

sudo: required

rust:
  - stable
  - beta
  - nightly

matrix:
  allow_failures:
    - rust: nightly

addons:
  apt:
    packages:
      - libcurl4-openssl-dev
      - libelf-dev
      - libdw-dev
      - cmake
      - gcc
      - binutils-dev
      - libiberty-dev

after_success: |
  wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz &&
  tar xzf master.tar.gz &&
  cd kcov-master &&
  mkdir build &&
  cd build &&
  cmake .. &&
  make &&
  make install DESTDIR=../../kcov-build &&
  cd ../.. &&
  rm -rf kcov-master &&
  for file in target/debug/examplerust-*[^\.d]; do mkdir -p "target/cov/$(basename $file)"; ./kcov-build/usr/local/bin/kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"; done &&
  bash <(curl -s https://codecov.io/bash) &&
  echo "Uploaded code coverage"
```
This will automatically run each executable and store the results in a
different directory. Codecov will automatically find the `cobertura.xml`
files that `kcov` generates and combine the results.

#### Other CI services
- Adjust the materials in the above example as necessary for your CI.
- Add CODECOV_TOKEN=<your repo token> to your CI's environment variables. (Don't store the raw token in your repo.)
- Run bash <(curl -s https://codecov.io/bash) after tests complete.

### Produce Coverage Reports

Run your tests with [kcov][6] in order to create the necessary coverage
reports. For example:

```
kcov --exclude-pattern=/.cargo,/usr/lib --verify target/cov target/debug/<PROJECT-NAME>-<hash>
```

`<PROJECT-NAME>` and `<hash>` are the appropriate project name and hash for
your executable.

The hash at the end may change if cargo generates different test
executables with the same name. If you are building your code
differently or without cargo, change the last two arguments
to kcov to respectively represent where you want the coverage to
be stored and which executable to run.

Attempting to run `kcov` with an executable argument ending in a wildcard
like `<PROJECT-NAME>-*` may result in incorrect coverage results as only a
single test executable will run. **For best results, run the kcov command
for each test executable and store the results in separate directories.**
Codecov will automatically find and upload the cobertura.xml files and
merge the coverage for you.

Note that setting the environment variable `RUSTFLAGS="-C link-dead-code"`
during tests build may improve coverage accuracy by preventing dead-code elimination.
Do not set this variable when creating release builds since it will increase
binary size.

After you've run the tests and created a cobertura.xml report, you can
use [the Codecov global uploader][4] to push that report to Codecov.
See below for further details.

Installing `kcov` is largely dependent on your operating system. It is
demonstrated to work on Linux systems but may not be fully compatible with
Windows or OS X. Please lookup the appropriate installation instructions.
The Travis CI example below demonstrates installing `kcov` on a Linux
computer.

## Caveats
### Private Repos
Add to your `.travis.yml` file.
```yml
after_success:
  - bash <(curl -s https://codecov.io/bash) -t uuid-repo-token
```
### `kcov` Version

The version of `kcov` that is distributed with your package manager may not
work with Rust binaries. You usually need to manually build the latest
master branch and run kcov from there. All of this is taken care of for you
in the `.travis.yml` file.
## Support

### Contact
- Intercom (in-app messanger)
- Email: [support@codecov.io](mailto:support@codecov.io)
- Slack: [slack.codecov.io](https://slack.codecov.io)
- [gh/codecov/support](https://github.com/codecov/support)

1. More documentation at https://docs.codecov.io
2. Configure codecov through the `codecov.yml`  https://docs.codecov.io/docs/codecov-yaml



[1]: https://codecov.io/
[5]: http://codecov.io/github/codecov/example-rust?branch=master
[6]: https://simonkagstrom.github.io/kcov/
[7]: http://stackoverflow.com/questions/35547710/does-rustdoc-generate-runnable-binaries
[8]: https://github.com/codecov/example-rust/issues
