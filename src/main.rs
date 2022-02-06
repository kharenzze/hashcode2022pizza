use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::time::Instant;

struct LocalTimer {
  start: Instant,
}

impl LocalTimer {
  fn new() -> Self {
    Self {
      start: Instant::now(),
    }
  }

  fn step(&mut self, name: &str) {
    let elapsed = self.start.elapsed();
    println!("{}: {:.2?}", name, elapsed);
    self.start = Instant::now();
  }
}

type TokenSet = HashSet<usize>;
type TokenMap = HashMap<String, usize>;
type TokenCount = HashMap<usize, usize>;

#[derive(Default, Debug)]
struct IngredientSet {
  set: TokenSet,
  hash: String,
}
#[derive(Default, Debug)]
struct Client {
  likes: IngredientSet,
  dislikes: IngredientSet,
}

#[derive(Default, Debug)]
struct SimpleCount {
  likes: TokenCount,
  dislikes: TokenCount,
}

#[derive(Default, Debug)]
struct Game {
  clients: Vec<Client>,
  tokens: TokenMap,
  simple_count: SimpleCount,
}

impl Game {
  fn ingest_line(&mut self, l: &str, like: bool) -> IngredientSet {
    let mut ingredient_iter = l.split_ascii_whitespace();
    let n: usize = ingredient_iter.next().unwrap().parse().unwrap();
    if n == 0 {
      return IngredientSet::default();
    }
    let info = IngredientSet {
      set: TokenSet::with_capacity(n),
      hash: String::new(),
    };
    let sorted: BinaryHeap<usize> = ingredient_iter
      .map(|ing| {
        let t = self.get_or_token(ing);
        let count = match like {
          true => &mut self.simple_count.likes,
          false => &mut self.simple_count.dislikes,
        };
        let &current = count.get(&t).unwrap_or(&0);
        count.insert(t, current + 1);
        t
      })
      .collect();
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

  fn get_solution_string(&self, tokens: &TokenSet) -> String {
    let mut solution = format!("{} ", tokens.len());
    for t in tokens {
      let pair = self.tokens.iter().find(|&(_, value)| t == value);
      let key = pair.unwrap().0;
      solution.push_str(key);
      solution.push_str(" ");
    }
    solution.pop();
    solution
  }

  fn simple_solution(&self) -> TokenSet {
    self
      .simple_count
      .likes
      .iter()
      .map(|(&key, &value)| {
        let &dislike_count = self.simple_count.dislikes.get(&key).unwrap_or(&0);
        (key, value as i32 - dislike_count as i32)
      })
      .filter(|&(_, value)| value >= 0)
      .map(|(key, _)| key)
      .collect()
  }

  fn init(&mut self, filename: &str) {
    let file: File = File::open(filename).expect(&format!("Cannot open file {}", filename));
    let reader = BufReader::new(file);
    let mut line_iter = reader.lines();
    let first: String = line_iter.next().unwrap().unwrap();
    let n: usize = first.parse().unwrap();
    for _i in 0..n {
      let line: String = line_iter.next().unwrap().unwrap();
      let likes = self.ingest_line(&line, true);
      let line: String = line_iter.next().unwrap().unwrap();
      let dislikes = self.ingest_line(&line, false);
      let client = Client { likes, dislikes };
      self.insert_client(client);
    }
  }

  fn measure(&self, tokens: &TokenSet) -> usize {
    self
      .clients
      .iter()
      .filter(|c| tokens.is_superset(&c.likes.set))
      .count()
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Invalid number of arguments")
  }
  let filename: &str = args.get(1).unwrap();
  let mut game: Game = Game::default();
  let mut timer = LocalTimer::new();
  game.init(filename);
  timer.step("Init");

  let tokens = game.simple_solution();
  timer.step("Tokens");
  let solution = game.get_solution_string(&tokens);
  timer.step("toString");

  println!("Points: {}", game.measure(&tokens));
  timer.step("Measure");

  let result_filename = format!("{}.result", filename);
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .open(&result_filename)
    .expect("Cannot open file");

  file
    .write_all(solution.as_bytes())
    .expect("Cannot write file");
}
