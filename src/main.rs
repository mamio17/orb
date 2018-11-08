extern crate ears;
use ears::{AudioController, Sound};

fn main() {
    let mut sound = Sound::new("assets/sample.wav").unwrap();
    sound.play();
    while sound.is_playing() {};
}
