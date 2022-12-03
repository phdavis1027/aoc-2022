use std::collections::HashSet;
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use math::round::floor;
use itertools::Itertools;

struct Rucksack {
    left: String,
    right: String
}

impl From<String> for Rucksack {
  fn from(s: String)
    -> Self
  {
    let halfway_point = floor((s.len() / 2) as f64, 0) as usize;
    let (left, right) = s.split_at(halfway_point);
    let left = left.to_string();
    let right = right.to_string();
    println!("{}-{}", left, right);
    Self {
      left,
      right
    }
  }
}

fn priority(c: char)
  -> usize
{
  let alph: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
  alph.find(c).expect("Could not find c") + 1
}

impl Rucksack {
    fn common_item_type(&self)
      -> Option<char>
    {
        let left_types = self.left.chars().collect::<HashSet<char>>();
        for c in self.right.chars() {
          if left_types.contains(&c) {
            return  Some(c);
          }
        }
        None
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn main() {
  let score: usize = read_lines("input.txt")
    .unwrap()
    .into_iter()
    .tuples::<(_, _, _)>()
    .map(|(a, b, c)|{
      let (a, b, c) = (a.unwrap(), b.unwrap(), c.unwrap());
      let common = a.chars()
                    .collect::<HashSet<char>>()
                    .intersection(
                      &b
                        .chars()
                        .collect::<HashSet<char>>()
                    )
                    .map(|c|{
                      *c
                    })
                    .collect::<HashSet<char>>();

    let common = common.intersection(&c.chars()
                                .collect::<HashSet<char>>())
                       .map(|c|{
                         *c
                       })
                       .collect::<HashSet<char>>();
    common.into_iter().next()
  })
  .map(|c|{
    priority(c.unwrap())
  })
  .sum();

  println!("Answer is {}", score);
}
