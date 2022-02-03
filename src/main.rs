use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

type TokenSet = HashSet<usize>;
struct Client {
  likes: TokenSet,
  dislikes: TokenSet,
}

#[derive(Default)]
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
      let token = self.get_or_token(ing);
      tokens.insert(token);
    }
    tokens
  }

  fn get_or_token(&mut self, t: &str) -> usize {
    let cached = self.tokens.get(t);
    if let Some(&c) = cached {
      return c;
    }
    let next_value = self.tokens.len() + 1;
    self.tokens.insert(t.to_string(), next_value);
    next_value
  }
  fn insert_client(&mut self, client: Client) {
    self.clients.push(client);
  }
}

fn init() -> Game {
  let stdin = stdin();
  let mut line_iter = stdin.lock().lines();
  let first: String = line_iter.next().unwrap().unwrap();
  let n: usize = first.parse().unwrap();
  let mut game: Game = Game::default();
  for _i in 0..n {
    let line: String = line_iter.next().unwrap().unwrap();
    let likes = game.ingest_line(&line);
    let line: String = line_iter.next().unwrap().unwrap();
    let dislikes = game.ingest_line(&line);
    let client = Client { likes, dislikes };
    game.insert_client(client);
  }
  game
}

fn main() {
  let game = init();
}
