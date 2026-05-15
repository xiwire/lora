/// Mono audio buffer with variable size. Really just for testing.
pub struct AudioBuffer<T>
where
    T: Copy
{
    /** Audio buffer */
    contents: Vec<T>,
}

impl<T> AudioBuffer<T> 
where
    T: Copy
{
    pub fn new() -> Self {
        AudioBuffer { contents: Vec::<T>::new() }
    }

    pub fn from_iter<I>(iter: I) -> Self 
    where
        I: IntoIterator<Item = T> 
    {
        AudioBuffer { contents: iter.into_iter().collect() }
    }

    pub fn peek(&self) -> Option<T> {
        self.contents.first().copied()
    }
}

//! Fixed size buffers

/// Fixed size buffer
struct FixedSizeMonoBuffer<T, const SIZE: usize> {
    contents: [T; SIZE],
}

struct InterleavedStereoBuffer<T> {

}
