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
  #[clap(long)]
  ascii: bool,

  /// Generator to use for the frames
  #[clap(value_enum, default_value_t = BowlingGeneratorType::Dice)]
  generator: BowlingGeneratorType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
#[clap(rename_all = "kebab-case")]
enum BowlingGeneratorType {
  Dice,
}

fn main() {
  let opt: Opt = Opt::parse();

  let rng = rand::thread_rng();
  let game: Game = match opt.generator {
    BowlingGeneratorType::Dice => Game::generate(generators::DiceGenerator::new(rng)),
  };

  let text = if opt.ascii {
    game.get_ascii_string()
  } else {
    game.get_ansi_string()
  };

  println!("{text}");
}
