use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type TokenSet = HashSet<usize>;

#[derive(Default, Debug)]
struct IngredientSet {
  set: TokenSet,
  hash: String,
}
struct Client {
  likes: IngredientSet,
  dislikes: IngredientSet,
}

#[derive(Default)]
struct Game {
  clients: Vec<Client>,
  tokens: HashMap<String, usize>,
}

impl Game {
  fn ingest_line(&mut self, l: &str) -> IngredientSet {
    let mut ingredient_iter = l.split_ascii_whitespace();
    let n: usize = ingredient_iter.next().unwrap().parse().unwrap();
    if n == 0 {
      return IngredientSet::default();
    }
    let info = IngredientSet {
      set: TokenSet::with_capacity(n),
      hash: String::new(),
    };
    let sorted: BinaryHeap<usize> = ingredient_iter.map(|ing| self.get_or_token(ing)).collect();
    let mut info: IngredientSet = sorted.into_iter().fold(info, |mut acc, token| {
      acc.set.insert(token);
      acc.hash.push_str(&format!("{}-", token));
      acc
    });
    info.hash.pop();
    info
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

  fn init(&mut self, filename: &str) {
    let file: File = File::open(filename).expect(&format!("Cannot open file {}", filename));
    let reader = BufReader::new(file);
    let mut line_iter = reader.lines();
    let first: String = line_iter.next().unwrap().unwrap();
    let n: usize = first.parse().unwrap();
    for _i in 0..n {
      let line: String = line_iter.next().unwrap().unwrap();
      let likes = self.ingest_line(&line);
      let line: String = line_iter.next().unwrap().unwrap();
      let dislikes = self.ingest_line(&line);
      let client = Client { likes, dislikes };
      self.insert_client(client);
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Invalid number of arguments")
  }
  let filename: &str = args.get(1).unwrap();
  let mut game: Game = Game::default();
  game.init(filename);
  println!("{:?}", &game.tokens)
}
