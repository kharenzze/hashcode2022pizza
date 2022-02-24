use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::fmt::Write as WWrite;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader, Write};
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
  contributors: Vec<String>,
}

#[derive(Default, Debug)]
struct Project {
  name: String,
  days: usize,
  score: usize,
  best_before: usize,
  n_contributors: usize,
  skills: HashMap<String, Skill>,
  skill_order: Vec<String>,
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
  skill_map: HashMap<String, Vec<String>>,
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
    let mut skill_map: HashMap<String, Vec<String>> = Default::default();
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
        let candidates = skill_map.get_mut(&skill_name);
        if let Some(c) = candidates {
          c.push(contributor_name.clone());
        } else {
          skill_map.insert(skill_name.clone(), vec![contributor_name.clone()]);
        }
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
      let mut sorted_skills: Vec<String> = vec![];
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
        sorted_skills.push(contributor_name.clone());
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
          skill_order: sorted_skills,
        },
      );
    }
    self.skill_map = skill_map;
    println!("{:?}", &self.skill_map);
  }

  fn greedy(&self) -> Vec<Plan> {
    let mut result = vec![];
    let mut projects_vec: Vec<String> = self.projects.keys().map(|x| x.clone()).collect();
    projects_vec.sort_by(|a, b| {
      let a_score = self.projects.get(a).unwrap().score;
      let b_score = self.projects.get(b).unwrap().score;
      b_score.cmp(&a_score)
    });
    for project_key in projects_vec {
      let project = self.projects.get(&project_key).unwrap();
      let mut candidates: HashSet<String> = Default::default();
      let mut candidates_vec: Vec<String> = Default::default();
      for req_name in project.skill_order.iter() {
        let req = project.skills.get(req_name).unwrap();
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
          candidates_vec.push(candidate.clone());
        }
      }
      if project.skill_order.len() == candidates.len() {
        result.push(Plan {
          project_name: project.name.clone(),
          contributors: candidates_vec,
        })
      }
    }
    result
  }
}

fn solution_to_string(r: Vec<Plan>) -> String {
  let mut sol = "".to_string();
  let l = r.len();
  if l != 0 {
    writeln!(&mut sol, "{}", l);
    for plan in r {
      writeln!(&mut sol, "{}", &plan.project_name);
      let arr: Vec<String> = plan.contributors.iter().map(|x| x.clone()).collect();
      writeln!(&mut sol, "{}", &arr.join(" "));
    }
  }

  sol
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

  let result = game.greedy();
  let solution = solution_to_string(result);

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
