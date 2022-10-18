use std::ops::{Add, Mul};

use crate::{
    input::Input,
    ops::{Amp, Mix},
    Synth,
};

enum Type {
    Simple,
    Freq {
        freq: Input,
        index: f32,
        last_sample: Option<f32>,
    },
}

pub struct Noise(Type);

impl Noise {
    pub fn new(freq: Option<Input>) -> Self {
        if let Some(freq) = freq {
            Self(Type::Freq {
                freq: freq,
                index: 0.0,
                last_sample: None,
            })
        } else {
            Self(Type::Simple)
        }
    }
}

impl Synth for Noise {
    fn get_sample(&mut self, rate: u32) -> Option<f32> {
        match &mut self.0 {
            Type::Simple => Some(rand::random()),
            Type::Freq {
                freq,
                index,
                last_sample,
            } => {
                let len = 1.0 / rate as f32;
                *index += len * freq.get_sample(rate)?;
                if *index >= 1.0 {
                    *index %= 1.0;
                    *last_sample = Some(rand::random());
                }

                if last_sample.is_none() {
                    *last_sample = Some(rand::random());
                }

                *last_sample
            }
        }
    }
}

impl<T: Into<Input>> Mul<T> for Noise {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> Add<T> for Noise {
    type Output = Mix;

    fn add(self, rhs: T) -> Self::Output {
        Mix::new(self, rhs)
    }
}
