// Generators getting exposed
mod dice_generator;

pub use dice_generator::DiceGenerator;

// Shared traits
const STRIKE: &str = "X";
const SPARE: &str = "/";

pub trait BowlingGenerator {
  fn generate_frame(&mut self) -> Frame;
  fn generate_last_frame(&mut self) -> LastFrame;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Frame {
  Strike,
  Spare { first: u16 },
  Open { first: u16, second: u16 },
}

impl Frame {
  #[allow(unused)]
  pub fn get_score(&self) -> u16 {
    self.get_sub_scores().into_iter().sum()
  }

  pub fn get_sub_scores(&self) -> Vec<u16> {
    match self {
      Frame::Strike => vec![10],
      Frame::Spare { first } => vec![*first, 10 - first],
      Frame::Open { first, second } => vec![*first, *second],
    }
  }

  pub fn sub_score_ansi_string(&self) -> String {
    match self {
      Frame::Strike => format!("{STRIKE}│ │"),
      Frame::Spare { first } => format!("{first}│{SPARE}│"),
      Frame::Open { first, second } => format!("{first}|{second}│"),
    }
  }

  pub fn sub_score_ascii_string(&self) -> String {
    match self {
      Frame::Strike => format!("{STRIKE}| |"),
      Frame::Spare { first } => format!("{first}|{SPARE}|"),
      Frame::Open { first, second } => format!("{first}|{second}|"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LastFrame {
  TripleStrike,
  DoubleStrikeOpen { third: u16 },
  StrikeSpare { second: u16 },
  StrikeOpen { second: u16, third: u16 },
  SpareStrike { first: u16 },
  SpareOpen { first: u16, third: u16 },
  Open { first: u16, second: u16 },
}

impl LastFrame {
  pub fn get_score(&self) -> u16 {
    self.get_sub_scores().into_iter().sum()
  }

  pub fn get_sub_scores(&self) -> Vec<u16> {
    match self {
      LastFrame::TripleStrike => vec![10, 10, 10],
      LastFrame::DoubleStrikeOpen { third } => vec![10, 10, *third],
      LastFrame::StrikeSpare { second } => vec![10, *second, 10 - second],
      LastFrame::StrikeOpen { second, third } => vec![10, *second, *third],
      LastFrame::SpareStrike { first } => vec![*first, 10 - first, 10],
      LastFrame::SpareOpen { first, third } => vec![*first, 10 - first, *third],
      LastFrame::Open { first, second } => vec![*first, *second],
    }
  }

  pub fn sub_score_ansi_string(&self) -> String {
    match self {
      LastFrame::TripleStrike => format!("{STRIKE}│{STRIKE}|{STRIKE}│"),
      LastFrame::DoubleStrikeOpen { third } => format!("{STRIKE}│{STRIKE}│{third}│"),
      LastFrame::StrikeSpare { second } => format!("{STRIKE}│{second}|{SPARE}│"),
      LastFrame::StrikeOpen { second, third } => format!("{STRIKE}|{second}|{third}│"),
      LastFrame::SpareStrike { first } => format!("{first}│{SPARE}│{STRIKE}│"),
      LastFrame::SpareOpen { first, third } => format!("{first}│{SPARE}│{third}│"),
      LastFrame::Open { first, second } => format!("{first}│{second}│ │"),
    }
  }

  pub fn sub_score_ascii_string(&self) -> String {
    match self {
      LastFrame::TripleStrike => format!("{STRIKE}|{STRIKE}|{STRIKE}|"),
      LastFrame::DoubleStrikeOpen { third } => format!("{STRIKE}|{STRIKE}|{third}|"),
      LastFrame::StrikeSpare { second } => format!("{STRIKE}|{second}|{SPARE}|"),
      LastFrame::StrikeOpen { second, third } => format!("{STRIKE}|{second}|{third}|"),
      LastFrame::SpareStrike { first } => format!("{first}|{SPARE}|{STRIKE}|"),
      LastFrame::SpareOpen { first, third } => format!("{first}|{SPARE}|{third}|"),
      LastFrame::Open { first, second } => format!("{first}|{second}| |"),
    }
  }
}
