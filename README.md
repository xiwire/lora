# Library Of Rust Audio

[![Crates.io](https://img.shields.io/crates/v/lora.svg)](https://crates.io/crates/lora)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**LORA** (**L**ibrary **O**f **R**ust **A**udio) is a high-performance _Digital Signal Processing_ (DSP) library written in Rust.

> [!NOTE]
> This library is primarily intended for my personal use. It is highly opinionated.

It provides a suite of reusable abstractions and primitives designed to streamline the development of audio software across platformsranging from high-level plugin architectures to low-level embedded systems.

The library focuses on ease-of-use, idiomatic constructions, efficiency, and platform independence, ensuring that DSP logic can be shared across different execution environments without modification.

# Features

- **Platform Agnostic:** Designed to operate without relying on a standard library (`no_std` compatible) and with C bindings, allowing it to run _anywhere_. Logic is decoupled from the actual platform it runs on to ensure consistent behavior across different projects. Library idioms allows writing in terms of _what you want to do_ instead of thinking what it's running on.
- **Generic Abstractions:** Flexible and easy to use implementations of common audio primitives (oscillators, filters, envelopes, delay lines, effects).
- **Zero-Cost Abstractions:** Leverages Rust's trait system to provide high-level APIs without sacrificing performance.

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

Using **LORA** normally goes like this:

```rust
use lora::dsp::{DelayLine, Send, LFO};

fn chorus(input_audio: f32, &output_audio: f32) {
    let mut delayline: DelayLine = DelayLine::new(sample_rate::seconds(0.2));
    let mut lfo: LFO = LFO::Triangle::new(
        LFO::Props{
            rate: sample_rate::hertz(1),
            amplitude: 0.017,
            offset: 0.017,
        }
    );
    lfo::modulate(delayline::params::time);
    let block_output = input_audio.copy();
    ProcessingBlock::new(input_audio, output_audio, [Send, delayline, Return])

    output_audio.copy(Mix(drywet, input_audio, block_output));
}
```

# Protyping with **LORA** 

I've made a companion tool [`run_lora_run`]() for quickly testing DSP algorithms. It allows you to quickly prototype and run your algorithms built with **LORA**.

# Contributing

Contributions are more than welcome! If you have a suggestion for a new DSP primitive or find a bug, please follow these steps:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/<feature_name>`).
3. Commit your changes.
4. Push to the branch and open a Pull Request.

# License

This project is licensed under the [MIT License](LICENSE).
