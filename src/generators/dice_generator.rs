use rand::{seq::SliceRandom, Rng};
use std::collections::LinkedList;

use crate::generators::{BowlingGenerator, Frame};

pub struct DiceGenerator<R: Rng> {
  rng: R,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum DiceSide {
  Blank,
  Pin,
  Spare,
  Strike,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum DiceType {
  Blank,
  Spare,
  Strike,
}

impl DiceType {
  pub fn roll<R: Rng>(self, r: &mut R) -> DiceSide {
    *self.get_dice_list().choose(r).unwrap()
  }

  pub fn get_dice_list(self) -> [DiceSide; 6] {
    use DiceSide::*;
    match self {
      Self::Blank => [Blank, Blank, Blank, Pin, Pin, Pin],
      Self::Spare => [Spare, Blank, Blank, Pin, Pin, Pin],
      Self::Strike => [Strike, Blank, Blank, Pin, Pin, Pin],
    }
  }
}

impl<R: Rng> DiceGenerator<R> {
  pub fn new(rng: R) -> Self {
    Self { rng }
  }

  fn get_all_dice() -> LinkedList<DiceType> {
    LinkedList::from([
      DiceType::Blank, // 4x blank
      DiceType::Blank,
      DiceType::Blank,
      DiceType::Blank,
      DiceType::Spare, // 3x spare
      DiceType::Spare,
      DiceType::Spare,
      DiceType::Strike, // 3x strike
      DiceType::Strike,
      DiceType::Strike,
    ])
  }
}

impl<R: Rng> BowlingGenerator for DiceGenerator<R> {
  fn generate_frame(&mut self) -> Frame {
    // First roll
    let mut first = 0;
    let mut second_roll_dice = LinkedList::new();
    for die in Self::get_all_dice() {
      match die.roll(&mut self.rng) {
        DiceSide::Strike => return Frame::Strike,
        DiceSide::Pin => second_roll_dice.push_back(die),
        DiceSide::Blank | DiceSide::Spare => {
          first += 1;
        },
      }
    }

    if first == 10 {
      return Frame::Strike;
    }

    // Second roll
    let mut second = 0;
    for die in second_roll_dice {
      match die.roll(&mut self.rng) {
        DiceSide::Spare => return Frame::Spare { first },
        DiceSide::Pin => {},
        DiceSide::Blank | DiceSide::Strike => {
          second += 1;
        },
      }
    }

    if first + second == 10 {
      Frame::Spare { first }
    } else {
      Frame::Open { first, second }
    }
  }
}
