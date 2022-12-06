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

//    let mut storage = Storage::new(4);
    let mut storage_part_2: Storage<(usize, &str)> = Storage::new(14);
    let dummy = 123456;

    let first = line
                  .split("")
                  .skip(1)
                  .enumerate()
                  .sliding_windows(&mut storage_part_2)
                  .map(|window|{
                      println!("{:?}", window);
                      let mut idx = 0;
                      match window
                        .into_iter()
                        .enumerate()
                        .map(|(i, (j, s))|{
                          if i == 13 {
                            idx = *j + 1;
                          }
                          *s
                        })
                        .collect::<HashSet<&str>>()
                        .len() == 14
                      {
                        true => {
                          idx
                        },
                        false => {
                          dummy
                        }
                      }
                  })
                  .min()
                  .unwrap();


  println!("First sequence starts at {}", first)
}
