use clap::{Parser, ValueEnum};
use game::Game;
use generators::BowlingGenerator;
use rand::Rng;

mod game;
mod generators;

/// Generate a random bowling game
#[derive(Debug, Parser)]
struct Opt {
  /// Output ASCII-only text instead of ANSI symbols
  #[clap(short, long)]
  ascii_text: bool,

  /// Generator to use for the frames
  #[clap(value_enum, default_value_t = BowlingGeneratorType::Dice)]
  generator: BowlingGeneratorType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
#[clap(rename_all = "kebab-case")]
enum BowlingGeneratorType {
  Dice,
}

impl BowlingGeneratorType {
  fn get_generator<'a, R: Rng + 'a>(self, rng: R) -> Box<dyn BowlingGenerator + 'a> {
    match self {
      Self::Dice => Box::new(generators::DiceGenerator::new(rng)),
    }
  }
}

fn main() {
  let opt: Opt = Opt::parse();

  let rng = rand::thread_rng();
  let game: Game = match opt.generator {
    BowlingGeneratorType::Dice => Game::generate(generators::DiceGenerator::new(rng)),
  };

  println!("{}", game.get_ansi_string());
}
