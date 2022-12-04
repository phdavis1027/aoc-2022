use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Default)]
struct Range {
  start: u32,
  end: u32
}

impl Range {
  fn contains(&self, other: &Self)
    -> bool
  {
    self.start <= other.start && self.end >= other.end
  }

  fn from_pair(pair: String)
    -> (Self, Self)
  {
    let pair = pair.splitn(2, ",").collect::<Vec<&str>>();
    (
      Range::from(
        String::from(
          *pair.get(0)
              .expect("oops")
        )
      ),
      Range::from(
        String::from(
          *pair.get(1)
              .expect("oops")
        )
      )
    )
  }

  fn intersects(&self, other: &Self)
    -> bool
  {
    self.start <= other.end && self.end >= other.start
  }
}



impl From<String> for Range {
  fn from(s: String)
    -> Self
  {
    let range_string = s.splitn(2, "-").collect::<Vec<&str>>();
    Self {
      start: range_string.get(0)
                         .expect("bad parse of start")
                         .parse::<u32>()
                         .expect("bad start string."),
      end: range_string.get(1)
                       .expect("bad parse of end")
                       .parse::<u32>()
                       .expect("bad end string.")
    }
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn main() {
  let count: u32 = read_lines("input.txt")
                  .unwrap()
                  .into_iter()
                  .map(
                    |s|{
                      Range::from_pair(s.unwrap())
                    }
                  )
                  .map(|(l, r)|{
                    match l.intersects(&r) {
                      true => 1,
                      false => 0
                    }
                  })
                  .sum();
  println!("{} pairs", count);
}
