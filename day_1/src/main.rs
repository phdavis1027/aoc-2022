use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn main() {
  let mut calories = Vec::new();

  if let Ok(lines) = read_lines("input.txt") {
    let mut current_calories = 0;
    for line in lines {
      if let Ok(ip) = line {
        if ip.is_empty() {
          calories.push(current_calories);
          current_calories = 0;
        } else {
          let added_calories = ip.parse::<i32>()
                                .expect("Oh hell.");
          current_calories += added_calories;
        }
      }
    }
  }

  calories.sort();
  calories.reverse();

  let mut top_3 = 0;
  for i in 0..3 {
    top_3 += calories.get(i).expect("Oops");
  }

  println!("The hardiest elf had {} calories.", calories.get(0).expect("Ah shit."));
  println!("The three hardiest elves together had {} calories", top_3);
}
