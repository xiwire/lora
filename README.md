# LORA

[![Crates.io](https://img.shields.io/crates/v/lora.svg)](https://crates.io/crates/lora)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**LORA** is a high-performance Digital Signal Processing (DSP) library written in Rust. It provides a suite of reusable abstractions and primitives designed to streamline the development of audio software, ranging from high-level plugin architectures to low-level embedded systems.

The library focuses on efficiency, type safety, and platform independence, ensuring that DSP logic can be shared across different execution environments without modification.

## 🚀 Key Features

- **Platform Agnostic:** Designed to operate without relying on a standard library (`no_std` compatible) where necessary.
- **Generic Abstractions:** Flexible implementations of common audio primitives (oscillators, filters, envelopes, delay lines, effects).
- **Sample-Rate Independent:** Logic is decoupled from the hardware sample rate to ensure consistent behavior across different projects.
- **Zero-Cost Abstractions:** Leverages Rust's trait system to provide high-level APIs without sacrificing performance.

## 🛠 Compatibility

LORA is specifically optimized for and tested against the following target platforms:

| Platform | Use Case | Integration Point |
| :--- | :--- | :--- |
| **nih-plug** | VST3/CLAP Plugins | High-level audio processing graphs |
| **Daisy Seed** | Embedded Hardware | Real-time DSP on ARM Cortex-M |
| **Max/MSP Externals** | Anything | Patching environment used standalone or
within Ableton Live  |

## 📦 Installation

Add `lora` to your `Cargo.toml`:

```toml
[dependencies]
lora = "0.1.0"
```

## 📖 Usage

### Basic Example
Below is a conceptual example of how to integrate a LORA abstraction into your audio loop:

```rust
use lora::dsp::Oscillator;

fn process_audio(sample_rate: f32) -> () {
    // Initialize a LORA oscillator
    let mut osc = Oscillator::new(sample_rate, 440.0);

    for _ in 0..buffer_size {
        let sample = osc.next_sample();
        // Output sample to buffer...
    }
}
```

## 🏗 Architecture

LORA is built around the concept of **DSP Units**. Each unit implements a standard set of traits that allow for:
- **Parameter Modulation:** Easy mapping of external inputs to internal DSP variables.
- **State Management:** Efficient handling of coefficients and buffers.
- **Interoperability:** Seamlessly piping the output of one unit into the input of another.

## 🤝 Contributing

Contributions are welcome! If you have a suggestion for a new DSP primitive or find a bug, please follow these steps:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/it-does-something`).
3. Commit your changes.
4. Push to the branch and open a Pull Request.

## 📜 License

This project is licensed under the [MIT License](LICENSE).
