# BeanCounter

About this project.

Add a badge for:
1. crates.io
2. github repository
3. license?
4. docs.rs
5. test status with Github Actions
6. Version?

# Installation

```bash
[dependencies]
beancounter = "0.1.0"
```

# Usage

# Roadmap

* Add a file to the documentation folder with metadata about struct size.
* Add a visualization of the struct size using a treemap.
* Add a visualization of a struct size using a piechart.

# Contributing

# MSRV

# Other Projects

The Rust ecosystem is rich and vibrant. These other projects might fit your needs better than `beancounter`. Many of them are mature, feature-rich, and robust, beyond `beancounter` in these dimensions.

* [`deepsize`](https://docs.rs/deepsize/latest/deepsize/). Deepsizeof can measure the amount of heap memory used at runtime by recursively walking the pointers in an object.
* [`datasize_derive`](https://lib.rs/crates/datasize_derive). This library provides an estimate of heap memory used, and implements the required sizing trait on much of the standard library.
* [`malloc_size_of`](https://github.com/servo/servo/tree/faf3a183f3755a9986ec4379abadf3523bd8b3c0/components/malloc_size_of): a Servo library that measures heap memory in a way that integrates with Firefox's memory profiler.
