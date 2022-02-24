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
struct Plan {
  project_name: String,
  contributors: HashSet<String>,
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
  fn init(&mut self, filename: &str) {
    let file: File = File::open(filename).expect(&format!("Cannot open file {}", filename));
    let reader = BufReader::new(file);
    let mut line_iter = reader.lines();
    let mut get_line = || line_iter.next().unwrap().unwrap();
    let first: String = get_line();
    let first_vec = first.split(" ").collect::<Vec<&str>>();
    let C = first_vec[0].parse::<usize>().unwrap();
    let P = first_vec[1].parse::<usize>().unwrap();
    for _i in 0..C {
      let contributor: String = get_line();
      let contributor_vec = contributor.split(" ").collect::<Vec<&str>>();
      let contributor_name: String = contributor_vec[0].to_string();
      let n_skills: usize = contributor_vec[1].parse::<usize>().unwrap();
      let mut skills_hashmap: HashMap<String, Skill> = HashMap::new();
      for _j in 0..n_skills {
        let skill: String = get_line();
        let skill_vec = skill.split(" ").collect::<Vec<&str>>();
        let skill_name: String = skill_vec[0].to_string();
        let skill_level: usize = skill_vec[1].parse::<usize>().unwrap();
        skills_hashmap.insert(
          skill_name.clone(),
          Skill {
            name: skill_name.clone(),
            level: skill_level,
          },
        );
      }
      self.contributors.insert(
        contributor_name.clone(),
        Contributor {
          name: contributor_name.clone(),
          skills: skills_hashmap,
        },
      );
    }
    for _i in 0..P {
      let project: String = get_line();
      let project_vec = project.split(" ").collect::<Vec<&str>>();
      let project_name: String = project_vec[0].to_string();
      let project_days: usize = project_vec[1].parse::<usize>().unwrap();
      let project_score: usize = project_vec[2].parse::<usize>().unwrap();
      let project_best_before: usize = project_vec[3].parse::<usize>().unwrap();
      let project_n_contributors: usize = project_vec[4].parse::<usize>().unwrap();
      let mut contributors_hashmap: HashMap<String, Skill> = HashMap::new();
      for _j in 0..project_n_contributors {
        let contributor: String = get_line();
        let contributor_vec = contributor.split(" ").collect::<Vec<&str>>();
        let contributor_name: String = contributor_vec[0].to_string();
        let level: usize = contributor_vec[1].parse::<usize>().unwrap();
        contributors_hashmap.insert(
          contributor_name.clone(),
          Skill {
            name: contributor_name.clone(),
            level: level,
          },
        );
      }
      self.projects.insert(
        project_name.clone(),
        Project {
          name: project_name.clone(),
          days: project_days,
          score: project_score,
          best_before: project_best_before,
          n_contributors: project_n_contributors,
          skills: contributors_hashmap,
        },
      );
    }
  }

  fn greedy(&self) -> Vec<Plan> {
    let mut result = vec![];
    for (name, p) in self.projects.iter() {
      let mut candidates: HashSet<String> = Default::default();
      for (_, req) in p.skills.iter() {
        let exist = self.contributors.iter().find(|(_, c)| {
          let skill_req = c.skills.get(&req.name);
          if let Some(r) = skill_req {
            if r.level >= req.level {
              return true;
            }
          }
          return false;
        });
        if let Some((candidate, _)) = exist {
          candidates.insert(candidate.clone());
        }
      }
      if p.skills.len() == candidates.len() {
        result.push(Plan { project_name: p.name.clone(), contributors: candidates })
      }
    }
    result
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
  println!("{:?}", game);
}
