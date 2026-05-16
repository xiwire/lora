# Library Of Rust Audio

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**LORA** (**L**ibrary **O**f **R**ust **A**udio) is a _Digital Signal Processing_ (DSP) library written in Rust.

> [!CAUTION]
> Work in progress, please ignore.

> [!NOTE]
> This library is primarily intended for my personal use. It is highly opinionated.

Reusable abstractions and primitives designed to facilitate the development of audio software across platforms.

Focuses on ease-of-use, idiomatic constructions, efficiency, and platform independence, trying to ensure that DSP logic can be shared across different targets without modification.

# Features

- **Platform Agnostic:** The main dsp blocks are designed to operate without relying on a standard library (`no_std` compatible) and with C bindings.
- **Generic Abstractions:** Flexible and easy to use implementations of common audio primitives (oscillators, filters, envelopes, delay lines, effects).

# Compatibility

**LORA** runs everywhere Rust can run or on anything that can run C through `cbindgen`. It is specifically optimized for the following use cases:

| Platform | Use Case | Specific plug |
| :--- | :--- | :--- |
| [**nih-plug**](https://codeberg.org/BillyDM/nih-plug) | VST3/CLAP Plugins | WIP |
| [**Electrosmith Daisy**](https://electro-smith.com/) | Hardware synths and effects | WIP |
| [**Max/MSP**](https://cycling74.com/products/max) | Optimized externals | WIP |
| [**Supercollider**](https://supercollider.github.io/) | UGens | WIP |

# Installation

Add `lora` to your `Cargo.toml`:

```toml
[dependencies]
lora = "0.1.0"
```

# Usage

LORA is built around the concept of **DSP Units**. Each unit implements a standard set of traits that allow for:
- **Parameter Modulation:** Easy mapping of external inputs to internal DSP variables.
- **State Management:** Efficient handling of coefficients and buffers.
- **Interoperability:** Seamlessly piping the output of one unit into the input of another.

Examples are in the [examples](/examples) folder.

# Protyping with **LORA** 

> [!WARNING]
> Not published yet.

I've made a companion tool [`lora-tauri`](https://github.com/xiwire/lora-tauri) for quickly testing DSP algorithms. It allows you to quickly prototype and run your algorithms built with **LORA**.

# Contributing

Contributions are more than welcome! If you have a suggestion for a new DSP primitive or find a bug, please follow these steps:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/<feature_name>`).
3. Commit your changes.
4. Push to the branch and open a Pull Request.

# License

This project is licensed under the [MIT License](LICENSE).
