use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::LinkedList;
use itertools::Itertools;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

enum ParsingPhase{
  Stacks,
  Moves
}

fn main() {

    let mut stacks = Vec::<Vec<char>>::with_capacity(2000);
    let mut current_parsing_phase = ParsingPhase::Stacks;
    let mut counter = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
              if ip.is_empty() {
                continue
              }
              if ip.as_bytes()[1] as char == '1' {
                continue
              }

              if ip.as_bytes()[0] as char == 'm' {
                current_parsing_phase = ParsingPhase::Moves;
              }

              match current_parsing_phase {
                ParsingPhase::Stacks => {
                  println!("{}", ip);
                  let mut cur_line_idx = 0;
                  let mut cur_stack = 0;
                  loop {
                    if cur_line_idx >= ip.len() { break }
                    let c = ip.as_bytes()[cur_line_idx] as char;
                    if c == '[' {
                      match (&mut stacks).get_mut(cur_stack) {
                        Some(stack) => {
                          stack.push(
                            (ip.as_bytes()[cur_line_idx+1] as char)
                          );
                          println!("Inserting {} into pre-existing stack at {}",
                            ip.as_bytes()[cur_line_idx+1] as char,
                            cur_stack
                            )
                        },
                        None => {
                          println!("Creating stack at {cur_stack} then inserting {} into it",
                            ip.as_bytes()[cur_line_idx+1] as char
                            );
                          stacks.insert(cur_stack, Vec::with_capacity(2000));
                          let mut stack = stacks.get_mut(cur_stack).unwrap();
                          stack.push(
                            ip.as_bytes()[cur_line_idx+1] as char
                          )
                        }
                      }
                    } else if stacks.get(cur_stack) == None {
                        println!("Creating empty stack at {}", cur_stack);
                        stacks.insert(cur_stack, Vec::with_capacity(2000));
                    }
                    cur_line_idx += 4;
                    cur_stack += 1;
                  }
                },
                ParsingPhase::Moves => {
                  if counter == 0 {
                    println!("---");
                    stacks.iter_mut().for_each(|stack|{
                      println!("{:?}", stack);
                      stack.reverse();
                    });
                    counter += 1;
                  }
                  let mut how_many = 0;
                  let mut from = 0;
                  let mut to = 0;
                  ip.split(" ").enumerate().for_each(|(i, a)|{
                    match i {
                      1 => {
                        how_many = a.parse::<usize>().unwrap();
                      },
                      3 => {
                        from = a.parse::<usize>().unwrap() - 1;
                      },
                      5 => {
                        to = a.parse::<usize>().unwrap() - 1;
                      }
                      _ => {}
                    }
                  });
                  let mut from = stacks.get_mut(from).unwrap();
                  let mut to_move = from.split_off(
                    from.len() - how_many
                  );

                  let mut to = stacks.get_mut(to).unwrap();
                  to.extend(to_move);
                }
              }
            }
        }
    }
    stacks.iter_mut().for_each(|stack|{
      print!("{}", stack.pop().unwrap());
    });
    print!("\n");
}
