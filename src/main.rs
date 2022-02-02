use std::io::{stdin, BufRead};

fn init() {
  let stdin = stdin();
  let mut line_iter = stdin.lock().lines();
  let first: String = line_iter.next().unwrap().unwrap();
  let n: usize = first.parse().unwrap();
  for i in 0..n {

  }
}

fn main() {
  init()
}
