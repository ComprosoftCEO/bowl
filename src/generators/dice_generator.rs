use rand::Rng;

use crate::generators::{BowlingGenerator, Frame, LastFrame};

pub struct DiceGenerator<R: Rng> {
  rng: R,
}

impl<R: Rng> DiceGenerator<R> {
  pub fn new(rng: R) -> Self {
    Self { rng }
  }
}

impl<R: Rng> BowlingGenerator for DiceGenerator<R> {
  fn generate_frame(&mut self) -> Frame {
    Frame::Spare { first: 9 }
  }

  fn generate_last_frame(&mut self) -> LastFrame {
    LastFrame::TripleStrike
  }
}
