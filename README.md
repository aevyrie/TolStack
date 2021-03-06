<p align="center">
  <img src="docs/logo.png" width="350">
</p>

## Overview

[![CI](https://img.shields.io/github/workflow/status/aevyrie/tolstack/Continuous%20integration/master)](https://github.com/aevyrie/tolstack/actions?query=workflow%3A%22Continuous+integration%22+branch%3Amaster)
[![dependency status](https://deps.rs/repo/github/aevyrie/tolstack/status.svg)](https://deps.rs/repo/github/aevyrie/tolstack)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://github.com/aevyrie/tolstack/blob/master/LICENSE)



TolStack is an open source tolerance analysis application for building and analyzing 1D geometric tolerance models. The goal of this tool is to help make tolerance analysis fast, intuitive, and explorable. Built with Rust using [`iced`](https://github.com/hecrj/iced).

[Read the TolStack user guide](https://aevyrie.github.io/tolstack/book/)

[Roadmap](https://github.com/aevyrie/tolstack/projects/1)

### Disclaimer

🚧 This application is in development, untested, unstable, and not ready for general use. 🚧

This software should only be used by engineers who are able to independently verify the correctness of its output. The software is provided as is, without warranty of any kind. The intent of this software is to aid in the exploration of tolerance analyses, not to replace existing methods of analysis or verification.

## Features

* Build one-dimensional tolerance stackups in a visual editor
* Evaluate and tune your tolerances with:
  * Monte Carlo analysis
  * RSS analysis
* Export results to CSV

### Screenshot

![Screenshot](docs/screenshot.png)

## Build Instructions

1. Install Rust via [Rustup](https://www.rust-lang.org/tools/install).
2. Clone the repository with `git clone https://github.com/aevyrie/tolstack.git`
3. From the `tolstack` directory, run `cargo run --release` to build and launch the application with compiler optimizations.

### Hardware and Software Requirements

* Note: make sure your graphics drivers are up to date!
* Linux/Windows: You will need a modern graphics card that supports Vulkan
  * Integrated graphics (Intel HDxxx) requires vulkan support, check [here](https://www.intel.com/content/www/us/en/support/articles/000005524/graphics.html)
* MacOS: the backend uses Metal, check [here](https://en.wikipedia.org/wiki/Metal_(API)#Supported_GPUs) for requirements

## License
This project is licensed under the [MIT license](https://github.com/aevyrie/tolstack/blob/master/LICENSE).

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in tolstack by you, shall be licensed as MIT, without any additional terms or conditions.
