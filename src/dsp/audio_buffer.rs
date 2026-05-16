use alloc::vec::Vec;
use core::default::Default;

pub trait AudioBuffer {
    type T;
    fn write(&mut self, data: &[Self::T]) -> usize;
    fn read(&mut self, output: &mut [Self::T]) -> usize;
}

/// Fixed size ring buffer
pub struct FixedMonoBuffer<T, const SIZE: usize> {
    contents: [T; SIZE],
    read_pos: usize,
    write_pos: usize,
    available: usize,
}

impl<T: Default + Copy, const SIZE: usize> FixedMonoBuffer<T, SIZE> {
    pub fn new() -> Self {
        Self {
            contents: [T::default(); SIZE],
            read_pos: 0,
            write_pos: 0,
            available: 0,
        }
    }

    pub fn available(&self) -> usize {
        self.available
    }

    /// Writes data from an iterator into the buffer.
    pub fn write_from_iter<I>(&mut self, iter: I) -> usize
    where
        I: IntoIterator<Item = T>,
    {
        let mut written = 0;
        for item in iter {
            if self.available >= SIZE {
                break;
            }
            self.contents[self.write_pos] = item;
            self.write_pos = (self.write_pos + 1) % SIZE;
            self.available += 1;
            written += 1;
        }
        written
    }
}

impl<T: Default + Copy, const SIZE: usize> AudioBuffer for FixedMonoBuffer<T, SIZE> {
    type T = T;

    fn write(&mut self, data: &[Self::T]) -> usize {
        let space_available = SIZE - self.available;
        let to_write = data.len().min(space_available);
        if to_write == 0 {
            return 0;
        }

        let first_chunk_size = (SIZE - self.write_pos).min(to_write);
        self.contents[self.write_pos..self.write_pos + first_chunk_size]
            .copy_from_slice(&data[..first_chunk_size]);

        if to_write > first_chunk_size {
            let second_chunk_size = to_write - first_chunk_size;
            self.contents[0..second_chunk_size].copy_from_slice(&data[first_chunk_size..to_write]);
        }

        self.write_pos = (self.write_pos + to_write) % SIZE;
        self.available += to_write;
        to_write
    }

    fn read(&mut self, output: &mut [Self::T]) -> usize {
        let to_read = output.len().min(self.available);
        if to_read == 0 {
            return 0;
        }

        let first_chunk_size = (SIZE - self.read_pos).min(to_read);
        output[..first_chunk_size]
            .copy_from_slice(&self.contents[self.read_pos..self.read_pos + first_chunk_size]);

        if to_read > first_chunk_size {
            let second_chunk_size = to_read - first_chunk_size;
            output[first_chunk_size..to_read].copy_from_slice(&self.contents[0..second_chunk_size]);
        }

        self.read_pos = (self.read_pos + to_read) % SIZE;
        self.available -= to_read;
        to_read
    }
}

/// Dynamic size ring buffer
pub struct DynamicMonoBuffer<T> {
    contents: Vec<T>,
    read_pos: usize,
    write_pos: usize,
    available: usize,
}

impl<T: Default + Copy> DynamicMonoBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            contents: alloc::vec![T::default(); capacity],
            read_pos: 0,
            write_pos: 0,
            available: 0,
        }
    }

    pub fn available(&self) -> usize {
        self.available
    }

    pub fn capacity(&self) -> usize {
        self.contents.len()
    }

    pub fn write_from_iter<I>(&mut self, iter: I) -> usize
    where
        I: IntoIterator<Item = T>,
    {
        let mut written = 0;
        let size = self.contents.len();
        for item in iter {
            if self.available >= size {
                break;
            }
            self.contents[self.write_pos] = item;
            self.write_pos = (self.write_pos + 1) % size;
            self.available += 1;
            written += 1;
        }
        written
    }
}

impl<T: Default + Copy> AudioBuffer for DynamicMonoBuffer<T> {
    type T = T;

    fn write(&mut self, data: &[Self::T]) -> usize {
        let size = self.contents.len();
        let space_available = size - self.available;
        let to_write = data.len().min(space_available);
        if to_write == 0 {
            return 0;
        }

        let first_chunk_size = (size - self.write_pos).min(to_write);
        self.contents[self.write_pos..self.write_pos + first_chunk_size]
            .copy_from_slice(&data[..first_chunk_size]);

        if to_write > first_chunk_size {
            let second_chunk_size = to_write - first_chunk_size;
            self.contents[0..second_chunk_size].copy_from_slice(&data[first_chunk_size..to_write]);
        }

        self.write_pos = (self.write_pos + to_write) % size;
        self.available += to_write;
        to_write
    }

    fn read(&mut self, output: &mut [Self::T]) -> usize {
        let size = self.contents.len();
        let to_read = output.len().min(self.available);
        if to_read == 0 {
            return 0;
        }

        let first_chunk_size = (size - self.read_pos).min(to_read);
        output[..first_chunk_size]
            .copy_from_slice(&self.contents[self.read_pos..self.read_pos + first_chunk_size]);

        if to_read > first_chunk_size {
            let second_chunk_size = to_read - first_chunk_size;
            output[first_chunk_size..to_read].copy_from_slice(&self.contents[0..second_chunk_size]);
        }

        self.read_pos = (self.read_pos + to_read) % size;
        self.available -= to_read;
        to_read
    }
}

/// Used for when audio is interleaved stereo.
pub struct InterleavedStereoBuffer {}

/// Used when audio is meant to be used in FFT/convolution algorithms.
pub struct PlanarStereoBuffer {}
