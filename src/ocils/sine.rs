use std::{
    f32::consts::PI,
    ops::{Add, Mul},
};

use crate::{
    input::Input,
    ops::{Amp, Mix},
    Synth,
};

pub struct Sine {
    freq: Input,
    index: f32,
}

impl Sine {
    pub fn new(freq: impl Into<Input>) -> Self {
        Self {
            freq: freq.into(),
            index: 0.0,
        }
    }
}

impl Synth for Sine {
    fn get_sample(&mut self, rate: u32) -> Option<f32> {
        let len = 1.0 / rate as f32;
        self.index += len * self.freq.get_sample(rate)?;
        self.index %= 1.0;
        let angle = self.index * 2.0 * PI;
        let ampl = angle.sin();
        Some(ampl)
    }
}

impl<T: Into<Input>> Mul<T> for Sine {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> Add<T> for Sine {
    type Output = Mix;

    fn add(self, rhs: T) -> Self::Output {
        Mix::new(self, rhs)
    }
}
