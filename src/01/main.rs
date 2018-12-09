use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");

fn part1() -> i32 {
  return INPUT.lines().fold(0, |sum, line| {
    let n = line
      .parse::<i32>()
      .unwrap_or_else(|e| panic!("{}: {}", line, e));

    return n + sum;
  });
}

fn part2() -> i32 {
  let mut score = HashSet::new();
  score.insert(0);

  let mut current_score = 0;
  for line in INPUT.lines().cycle() {
    let n = line
      .parse::<i32>()
      .unwrap_or_else(|e| panic!("{}: {}", line, e));

    current_score = current_score + n;

    if score.contains(&current_score) {
      return current_score;
    }

    score.insert(current_score);
  }

  return -1;
}

fn main() {
  println!("{}", part1());
  println!("{}", part2());
}

#[cfg(test)]
mod test;
