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

#[derive(Default, Debug)]
struct Skill {
  name: String,
  level: usize,
}

#[derive(Default, Debug)]
struct Project {
  name: String,
  days: usize,
  score: usize,
  best_before: usize,
  n_contributors: usize,
  skills: HashMap<String, Skill>,
}

#[derive(Default, Debug)]
struct Contributor {
  name: String,
  skills: HashMap<String, Skill>,
}

#[derive(Default, Debug)]
struct Game {
  projects: HashMap<String, Project>,
  contributors: HashMap<String, Contributor>,
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
      //let pair = self.tokens.iter().find(|&(_, value)| t == value);
      let ingredient = self.reversed_tokens.get(t).expect("token must exist");
      solution.push_str(ingredient);
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

  fn advanced_solution(&self) -> TokenSet {
    let simple = self.simple_solution();
    let mut dislike_options: BinaryHeap<(usize, usize)> = simple
      .iter()
      .map(|&token| {
        (
          self
            .simple_count
            .dislikes
            .get(&token)
            .unwrap_or(&0)
            .to_owned(),
          token,
        )
      })
      .collect();
    let mut solution: TokenSet = simple.clone();
    let mut best_score = self.measure(&solution);

    while let Some((_, token)) = dislike_options.pop() {
      solution.remove(&token);
      let score = self.measure(&solution);
      if score > best_score {
        best_score = score;
      } else {
        solution.insert(token);
      }
    }

    let all: TokenSet = self.tokens.values().cloned().collect();

    let rest: TokenSet = all.difference(&solution).cloned().collect();

    for token in rest {
      solution.insert(token);
      let score = self.measure(&solution);
      if score > best_score {
        best_score = score;
      } else {
        solution.remove(&token);
      }
    }

    solution
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

    self.reversed_tokens = self.tokens.iter().fold(
      ReversedTokenMap::with_capacity(self.tokens.len()),
      |mut acc, (key, &value)| {
        acc.insert(value, key.clone());
        acc
      },
    )
  }

  fn measure(&self, tokens: &TokenSet) -> usize {
    self
      .clients
      .iter()
      .filter(|c| {
        tokens.is_superset(&c.likes.set) && tokens.intersection(&c.dislikes.set).count() == 0
      })
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

  game.simple_solution();
  timer.step("Tokens");
  let tokens = game.advanced_solution();
  timer.step("Tokens Advanced");
  let solution = game.get_solution_string(&tokens);
  timer.step("toString");

  println!("Points: {}", game.measure(&tokens));
  timer.step("Measure");

  let result_filename = format!("{}.result", filename);
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(&result_filename)
    .expect("Cannot open file");

  file
    .write_all(solution.as_bytes())
    .expect("Cannot write file");
}
