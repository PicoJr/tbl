# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0-alpha.1](https://crates.io/crates/tbl/1.1.0-alpha.1) Jul 14, 2020

* Support for overlapping intervals.

see `cargo run --example datetime`

## [1.1.0-alpha](https://crates.io/crates/tbl/1.1.0-alpha) Jun 19, 2020

* Breaking API Change: provide labels when intersection is found: `TBLError::Intersection(Option<L>, Option<L>)`.

## [1.0.0-alpha](https://crates.io/crates/tbl/1.0.0-alpha) Jun 13, 2020

* add multiline, `Renderer::render` now returns `Vec<String>`.
* add multiline example.
* remove `BlockRenderer` (**breaking change**) see [examples](examples).

## [0.1.1](https://crates.io/crates/tbl/0.1.1) May 27, 2020

* support empty intervals
* fix incorrect padding


## [0.1.0](https://crates.io/crates/tbl/0.1.0) May 25, 2020

* initial release
