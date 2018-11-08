extern crate ears;
use ears::{AudioController, Sound};

fn main() {
    let mut sound = Sound::new("AbJo_Drum_and_Freak.wav").unwrap();
    sound.play();
    while sound.is_playing() {};
}

