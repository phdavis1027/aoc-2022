use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq)]
enum Move {
  Rock,
  Paper,
  Scissors
}
struct Round {
  you: Move,
  opp: Move
}

impl From<String> for Round {
  fn from(round: String) -> Self {
    let mut split = round.split(" ");
    let _opp = split.next().expect("Bad _opp.");
    let _you = split.next().expect("Bad _you.");

    Self {
      opp: match _opp {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        &_ => panic!("Bad value for opp")
      },
      /*
       *
       *
       * THIS IS THE FIRST PART
      you: match _you {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        &_ => panic!("Bad value for you")
      }
      */
      you: match _you {
        "X" => match _opp {
          "A" => Move::Scissors,
          "B" => Move::Rock,
          "C" => Move::Paper,
          &_ => panic!("Bad value for opp")
        }
        "Y" => match _opp {
          "A" => Move::Rock,
          "B" => Move::Paper,
          "C" => Move::Scissors,
          &_ => panic!("Bad value for opp")
        }
        "Z" => match _opp {
          "A" => Move::Paper,
          "B" => Move::Scissors,
          "C" => Move::Rock,
          &_ => panic!("Bad value for opp")
        },
        &_ => panic!("Bad value for you")
      }
    }
  }
}

fn won(opp: &Move, you: &Move)
  -> bool
{
  (*opp == Move::Rock && *you == Move::Paper)
      || (*opp == Move::Paper && *you == Move::Scissors)
      || (*opp == Move::Scissors && *you == Move::Rock)
}

impl Round {
  fn score(&self)
    -> u32
  {
    let mut score = match self.you {
      Move::Rock => 1,
      Move::Paper => 2,
      Move::Scissors => 3
    };

    if self.opp == self.you {
      score += 3;
    } else if won(&self.opp, &self.you) {
      score += 6
    }

    score
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn main() {
  let mut total_score = 0;
  if let Ok(lines) = read_lines("input.txt") {
    for line in lines {
      if let Ok(ip) = line {
        let current_round = Round::from(ip);
        total_score += current_round.score();
      }
    }
  }

  println!("Your total score was {}", total_score);
}
