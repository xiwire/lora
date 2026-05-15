use crate::dsp::constants;

pub struct Sine {
    pos: f32,
    freq: f32,
}

impl Sine {
    fn new(freq: f32) -> Self {
        Sine{ pos: 0.0, freq: freq }
    }

    fn process(&mut self, sample_rate: usize) -> f32 {
        self.pos += self.freq / (sample_rate as f32);
        self.pos %= 1.0;

        (self.pos * 2.0 * constants::PI).sin()
    }
}
