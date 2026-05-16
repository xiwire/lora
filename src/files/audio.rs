use std::path::Path;

use wave_stream::{open_wav::OpenWav};

use crate::dsp::buffer::{AudioBuffer, DynamicMonoBuffer};

pub fn from_path(filepath: &str) -> DynamicMonoBuffer<f32> {
    // TODO: should actually accept multiple audio formats
    use wave_stream::{read_wav_from_file_path, wave_reader::StreamOpenWavReader};

    let wav_reader = read_wav_from_file_path(Path::new(filepath)).unwrap();
    let size = wav_reader.len_samples();

    let mut buffer = DynamicMonoBuffer::<f32>::new(size);

    let stream  = wav_reader
        .get_stream_f32_reader()
        .unwrap()
        .into_iter()
        .map(|i| i.unwrap().front_left.unwrap());

    let vector = Vec::<f32>::from_iter(stream);

    buffer.write(vector.as_slice());

    buffer
}
