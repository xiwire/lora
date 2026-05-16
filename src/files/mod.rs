//! For desktop files.
#[cfg(feature = "desktop")]

/// For managing audio files (.WAV, .MP3, .AIFF) and the such and using them as
/// [`crate::dsp::buffer::AudioBuffer`].
pub mod audio;

/// For parsing JSON files.
pub mod json;
