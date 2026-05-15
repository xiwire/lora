use std::path::Path;

use crate::dsp::audio_buffer::AudioBuffer;

#[cfg(feature = "desktop")]
pub fn from_path(filepath: &str) -> AudioBuffer::<f32> {
    use wave_stream::{read_wav_from_file_path, wave_reader::StreamOpenWavReader};

    let wav_reader = read_wav_from_file_path(Path::new(filepath)).unwrap();
    let stream = wav_reader.get_stream_f32_reader().unwrap().into_iter().map(|i| i.unwrap().front_left.unwrap());

    AudioBuffer::<f32>::from_iter(stream)
}
