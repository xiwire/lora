#[cfg(feature = "desktop")]
pub mod audio_buffer_test {

    use crate::{files};
    use crate::dsp::buffer::*;

    #[test]
    fn test_audio_buffer() {
        let audio_file = files::audio::from_path("/home/xiwire/Downloads/854137__vintprox__mastodon-trumpet.wav");
        let expected_capacity = 48001;
        let actual_capacity = audio_file.contents.capacity().get();

        assert_eq!(actual_capacity, expected_capacity);
    }

    #[test]
    fn can_read_into_vec() {
        let audio_file = files::audio::from_path("/home/xiwire/Downloads/854137__vintprox__mastodon-trumpet.wav");
        let vector = Vec::<f32>::with_capacity(audio_file.contents.capacity().get());

        let all_match: bool = vector.iter().zip(audio_file.contents.iter()).map(|(v, b)| v == b).all(|c| c);

        assert!(all_match)
    }
}
