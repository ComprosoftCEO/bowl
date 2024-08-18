# Bowl

Stupid command-line utility to generate a random bowling game

## Compiling

[Install Rust](https://www.rust-lang.org/learn/get-started), then build the program using:

```bash
cargo build
```

## Usage

```
Generate a random bowling game

Usage: bowl [OPTIONS] [GENERATOR]

Arguments:
  [GENERATOR]  Generator to use for the frames [default: dice]

Options:
      --ascii  Output ASCII-only text instead of ANSI symbols
  -h, --help   Print help
```

The only random generator supported right now is `dice` which is based on the [Bowling Dice Game](https://boardgamegeek.com/boardgame/3934/bowling-dice). But at some point I might add other generator types.

### ANSI Output

```bash
bowl
```

```text
┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐
│3|4│3│/│5│/│5│/│5|2│3|3│5|2│2|2│4|5│4│3│ │
│ └─┤ └─┤ └─┤ └─┤ └─┤ └─┤ └─┤ └─┤ └─┤ └─┴─┤
│  7│ 22│ 37│ 52│ 59│ 65│ 72│ 76│ 85│   92│
└───┴───┴───┴───┴───┴───┴───┴───┴───┴─────┘
```

### ASCII Output

```
bowl --ascii
```

```text
___________________________________________
|3|5|X| |6|1|3|/|2|3|4|3|X| |X| |X| |X|X|5|
| '-| '-| '-| '-| '-| '-| '-| '-| '-| '---|
|  8| 25| 32| 44| 49| 56| 86|116|146|  171|
|___|___|___|___|___|___|___|___|___|_____|
```

## Why?

It sounded fun to have a command-line program that could bowl for you.
