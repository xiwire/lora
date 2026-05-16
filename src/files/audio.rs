use std::path::Path;

use wave_stream::open_wav::OpenWav;

use crate::dsp::audio_buffer::DynamicMonoBuffer;

pub fn from_path(filepath: &str) -> DynamicMonoBuffer<f32> {
    use wave_stream::{read_wav_from_file_path, wave_reader::StreamOpenWavReader};

    let wav_reader = read_wav_from_file_path(Path::new(filepath)).unwrap();
    let size = wav_reader.len_samples();
    let stream = wav_reader
        .get_stream_f32_reader()
        .unwrap()
        .into_iter()
        .map(|i| i.unwrap().front_left.unwrap());

    let mut buffer = DynamicMonoBuffer::<f32>::new(size);

    buffer.write_from_iter(stream);

    buffer
}
