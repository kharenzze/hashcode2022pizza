use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

type TokenSet = HashSet<usize>;
struct Client {
  likes: TokenSet,
  dislikes: TokenSet,
}

struct Game {
  clients: Vec<Client>,
  tokens: HashMap<String, usize>,
}

impl Game {
  fn ingest_line(&mut self, l: &str) -> TokenSet {
    let mut ingredient_iter = l.split_ascii_whitespace();
    let n: usize = ingredient_iter.next().unwrap().parse().unwrap();
    if n == 0 {
      return TokenSet::new();
    }
    let mut tokens = TokenSet::with_capacity(n);
    for _i in 0..n {
      let ing: &str = ingredient_iter.next().unwrap();
      let token = self.get_token(ing);
      tokens.insert(token);
    }
    tokens
  }

  pub fn get_token(&self, t: &str) -> usize {
    1
  }
}

fn init() {
  let stdin = stdin();
  let mut line_iter = stdin.lock().lines();
  let first: String = line_iter.next().unwrap().unwrap();
  let n: usize = first.parse().unwrap();
  for _i in 0..n {}
}

fn main() {
  init()
}
