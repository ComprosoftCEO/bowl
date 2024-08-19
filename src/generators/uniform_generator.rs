use rand::{seq::IteratorRandom, Rng};

use crate::generators::{BowlingGenerator, Frame};

pub struct UniformGenerator<R: Rng> {
  rng: R,
}

impl<R: Rng> UniformGenerator<R> {
  pub fn new(rng: R) -> Self {
    Self { rng }
  }
}

impl<R: Rng> BowlingGenerator for UniformGenerator<R> {
  fn generate_frame(&mut self) -> Frame {
    let first = (0..=10).choose(&mut self.rng).unwrap();
    if first == 10 {
      return Frame::Strike;
    }

    let second = (0..=(10 - first)).choose(&mut self.rng).unwrap();
    if first + second == 10 {
      return Frame::Spare { first };
    }

    Frame::Open { first, second }
  }
}
