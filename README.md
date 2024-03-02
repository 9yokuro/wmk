# Wmk - A file creation tool written in rust

![workflow_rust](https://github.com/9yokuro/wmk/actions/workflows/build.yml/badge.svg)
![Crates.io Version](https://img.shields.io/crates/v/wmk)

## Features
- Colorful, easy-to-read output
- Record history

With mkdir and touch:
```console
$ mkdir --parents a/b/c/d/e/f/
$ touch a/b/c/d/e/f/g.txt
```
With wmk:
```console
$ wmk a/b/c/d/e/f/g.txt
```

## Installation
Run the following Cargo command:
```console
$ cargo install wmk
```

## Usage
Create files:
```console
$ wmk foo.txt bar.txt ...
```

Create directories:
```console
$ wmk --directory foo_dir bar_dir ...
```

Show history:
```console
$ wmk --show-history

02 Mar 10:37   exists   /absolute/path/to/foo.txt
02 Mar 10:37   exists   /absolute/path/to/bar.txt
02 Mar 10:37   exists   /absolute/path/to/foo_dir
02 Mar 10:37   exists   /absolute/path/to/bar_dir
```
directories are shown in blue.

Delete history interactively:
```console
$ wmk --delete-history
```

Clear history:
```console
$ wmk --clear-history
```

### Options
```console
-c, --clear-history   Clear history
-D, --delete-history  Delete history interactively
-d, --directory       Create directories instead of files
-q, --quiet           Do not print log messages
-s, --show-history    Show history
-h, --help            Print help
-V, --version         Print version
```
## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## TODO
- add `--no-record` option
