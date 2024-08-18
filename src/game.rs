use std::{array, iter};

use crate::generators::{BowlingGenerator, Frame, LastFrame};

/// Represents an entire game of bowling
pub struct Game {
  frames: [Frame; 9],
  last_frame: LastFrame,
}

impl Game {
  pub fn generate<G: BowlingGenerator>(mut gen: G) -> Self {
    let frames = array::from_fn(|_| gen.generate_frame());
    let last_frame = gen.generate_last_frame();
    Game { frames, last_frame }
  }

  pub fn get_ansi_string(&self) -> String {
    let top_row = "┌".to_string() + &"─┬─┬".repeat(9) + "─┬─┬─┐";

    let sub_scores_row = "│".to_string()
      + &self.frames.iter().map(Frame::sub_score_ansi_string).collect::<String>()
      + &self.last_frame.sub_score_ansi_string();

    let middle_row = "│".to_string() + &" └─┤".repeat(9) + " └─┴─┤";

    let cumulative_scores_row =
      self
        .get_cumulative_frame_scores()
        .into_iter()
        .enumerate()
        .fold("│".to_string(), |acc, (i, score)| {
          let len = if i == 9 { 5 } else { 3 }; // Special case for last frame
          acc + &format!("{score:>len$}│", len = len)
        });

    let bottom_row = "└".to_string() + &"───┴".repeat(9) + "─────┘";

    [top_row, sub_scores_row, middle_row, cumulative_scores_row, bottom_row].join("\n")
  }

  pub fn get_ascii_string(&self) -> String {
    let top_row = "_".to_string() + &"____".repeat(9) + "______";

    let sub_scores_row = "|".to_string()
      + &self
        .frames
        .iter()
        .map(Frame::sub_score_ascii_string)
        .collect::<String>()
      + &self.last_frame.sub_score_ascii_string();

    let middle_row = "|".to_string() + &" '-|".repeat(9) + " '---|";

    let cumulative_scores_row =
      self
        .get_cumulative_frame_scores()
        .into_iter()
        .enumerate()
        .fold("|".to_string(), |acc, (i, score)| {
          let len = if i == 9 { 5 } else { 3 }; // Special case for last frame
          acc + &format!("{score:>len$}|", len = len)
        });

    let bottom_row = "|".to_string() + &"___|".repeat(9) + "_____|";

    [top_row, sub_scores_row, middle_row, cumulative_scores_row, bottom_row].join("\n")
  }

  fn get_cumulative_frame_scores(&self) -> [u16; 10] {
    let mut cumulative_score = 0;

    let mut frame_scores = self.get_frame_scores();
    for frame_score in &mut frame_scores {
      *frame_score += cumulative_score;
      cumulative_score = *frame_score;
    }

    frame_scores
  }

  // Ugly but gets the job done :)
  fn get_frame_scores(&self) -> [u16; 10] {
    let frame_sub_score_lists = self
      .frames
      .iter()
      .map(Frame::get_sub_scores)
      .chain(iter::once(&self.last_frame).map(LastFrame::get_sub_scores));

    let mut frame_sub_score_index = [0usize; 10];
    let mut current_index = 0;
    let frame_sub_scores: Vec<_> = frame_sub_score_lists
      .enumerate()
      .flat_map(|(i, frame_scores)| {
        frame_sub_score_index[i] = current_index;
        current_index += frame_scores.len();
        frame_scores
      })
      .collect();

    let mut frame_scores = [0; 10];
    for i in 0..9 {
      let frame_sub_score_index = frame_sub_score_index[i];
      frame_scores[i] = match self.frames[i] {
        Frame::Open { first, second } => first + second,
        Frame::Spare { .. } => 10 + frame_sub_scores[frame_sub_score_index + 2],
        Frame::Strike => 10 + frame_sub_scores[frame_sub_score_index + 1] + frame_sub_scores[frame_sub_score_index + 2],
      }
    }

    frame_scores[9] = self.last_frame.get_score();

    frame_scores
  }
}
