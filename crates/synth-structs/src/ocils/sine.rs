use std::{f32::consts::PI, ops};

use crate::{
    ops::{Add, Amp},
    Context, Input, Synth,
};

#[derive(Clone)]
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
    fn sample(&mut self, context: Context) -> Option<f32> {
        let angle = self.index * 2.0 * PI;
        let ampl = angle.sin();

        let len = 1.0 / context.rate as f32;
        self.index += len * self.freq.sample(context)?;
        self.index %= 1.0;

        Some(ampl)
    }
}

impl<T: Into<Input>> ops::Mul<T> for Sine {
    type Output = Amp;

    fn mul(self, rhs: T) -> Self::Output {
        Amp::new(self, rhs)
    }
}

impl<T: Into<Input>> ops::Add<T> for Sine {
    type Output = Add;

    fn add(self, rhs: T) -> Self::Output {
        Add::new(self, rhs)
    }
}
