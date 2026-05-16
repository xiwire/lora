pub use ringbuf::traits::*;
use ringbuf::{HeapRb, StaticRb};
use core::default::Default;

pub trait AudioBuffer {
    type T;

    fn write(&mut self, data: &[Self::T]) -> usize;
    fn read(&mut self, output: &mut [Self::T]) -> usize;
    fn clear(&mut self) -> usize;
}

/// Fixed size ring buffer
pub struct FixedMonoBuffer<T, const SIZE: usize> 
where 
    T: Default + Copy
{
    pub contents: StaticRb<T, SIZE>,
}

pub struct DynamicMonoBuffer<T> 
where 
    T: Default + Copy
{
    pub contents: HeapRb<T>,
}

impl<T> DynamicMonoBuffer<T>
where 
    T: Default + Copy
{
   pub fn new(capacity: usize) -> Self {
      Self { contents: HeapRb::<T>::new(capacity) } 
   } 
}

impl<T> AudioBuffer for DynamicMonoBuffer<T>
where 
    T: Default + Copy
{
    type T = T;
    fn write(&mut self, data: &[Self::T]) -> usize {
        self.contents.push_slice(data)
    }

    fn read(&mut self, output: &mut [Self::T]) -> usize {
        self.contents.pop_slice(output)
    }

    fn clear(&mut self) -> usize {
        self.contents.clear()
    }
}

/// Used for when audio is interleaved stereo.
pub struct InterleavedStereoBuffer {}

/// Used when audio is meant to be used in FFT/convolution algorithms.
pub struct PlanarStereoBuffer {}
