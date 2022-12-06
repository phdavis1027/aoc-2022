use std::fs::File;
use std::io::{self, BufRead};
use itertools::Itertools;
use std::path::Path;
use std::collections::HashSet;
use sliding_windows::IterExt;
use sliding_windows::Storage;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn main() {
    let line = read_lines("input.txt")
        .unwrap()
        .next()
        .unwrap()
        .unwrap();

    let mut storage = Storage::new(4);
    let dummy = 123456;

    let first = line
                  .split("")
                  .skip(1)
                  .enumerate()
                  .sliding_windows(&mut storage)
                  .map(|window|{
                      println!("{:?}", window);
                      *window
                        .into_iter()
                        .tuples::<(_, _, _, _)>()
                        .map(|((_i, a), (_j, b), (_k, c), (l, d))| {
                          match vec![a, b, c, d]
                                .into_iter()
                                .map(|s|{
                                  *s
                                })
                                .collect::<HashSet<&str>>()
                                .len() == 4 {
                                  true => {
                                   l + 1
                                  },
                                  false => {
                                    &dummy
                                  }
                                }
                        })
                        .min()
                        .unwrap()
                  })
                  .min()
                  .unwrap();


  println!("First sequence starts at {}", first)
}
