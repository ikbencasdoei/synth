use synth::{ocils::Sine, Player};

fn main() {
    Player::play(Sine::new(200) * 0.5)
}
