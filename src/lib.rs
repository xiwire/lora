//! LORA is a DSP library.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// For all the proper DSP thingies. 
pub mod dsp;

/// For manipulating files in storage. Only applies to `feature=desktop` builds. For managing files
/// in embdeded system use the `storage` mod.
pub mod files;
