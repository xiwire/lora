// use std::collections::HashSet;
//
// use crate::dsp::audio_buffer::AudioBuffer;

pub trait ProcessingUnit {
    fn process(&mut self) -> ();
    fn output_into(&self, other: dyn ProcessingUnit) -> ();
    fn input_from(&self, other: dyn ProcessingUnit) -> ();
    // fn to_buffer(&self, buffer: &dyn AudioBuffer) -> ();
}

pub struct ProcessingUnitChain {}
