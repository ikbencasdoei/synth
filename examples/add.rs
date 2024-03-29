use rand::{thread_rng, Rng};
use synth::{ocils::Table, ops::Add, Input, Player};

fn main() {
    let mut rng = thread_rng();

    Player::play(Add::from_vec(
        (0..10)
            .map(|_| Input::from(Table::with_sine(1000, rng.gen_range(1.0..250.0)) * 0.05))
            .collect(),
    ));
}
