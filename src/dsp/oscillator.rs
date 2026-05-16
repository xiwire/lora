use crate::dsp::constants;

#[allow(unused_imports)]
#[cfg(feature = "embedded")]
use micromath::F32Ext;

pub struct Sine {
    pos: f32,
    freq: f32,
}

impl Sine {
    pub fn new(freq: f32) -> Self {
        Sine{ pos: 0.0, freq: freq }
    }

    pub fn process(&mut self, sample_rate: usize) -> f32 {
        self.pos += self.freq / (sample_rate as f32);
        self.pos %= 1.0;

        f32::sin(self.pos * 2.0 * constants::PI)
    }
}
