use std::collections::HashMap;

static INPUT: &str = include_str!("input.txt");

fn part2() -> String {
  let mut x: Vec<&str> = INPUT.lines().collect();
  x.sort_by(|a, b| a.cmp(b));

  // should be a nicer way to iterate with lookback
  let mut previous: Option<&str> = None;
  for line in x {
    match previous {
      Some(s) => {
        let mut difference = 0;
        let mut hello = String::new();

        for (a, b) in line.chars().zip(s.chars()) {
          if a != b {
            difference += 1;
          } else {
            hello.push(a)
          }
        }
        if difference == 1 {
          return hello;
        }
      }
      None => (),
    }

    previous = Some(line)
  }
  panic!("Not found");
}

fn part1() -> i32 {
  let mut with_2 = 0;
  let mut with_3 = 0;

  for line in INPUT.lines() {
    let mut count = HashMap::new();

    let mut did_count_as_2 = false;
    let mut did_count_as_3 = false;

    for c in line.chars() {
      let r = count.entry(c).or_insert(0);
      *r += 1;
    }

    for (_, n) in &count {
      if *n == 2 && !did_count_as_2 {
        with_2 += 1;
        did_count_as_2 = true
      }

      if *n == 3 && !did_count_as_3 {
        with_3 += 1;
        did_count_as_3 = true
      }
    }
  }

  return with_2 * with_3;
}

fn main() {
  println!("{}", part1());
  println!("{}", part2());
}

#[cfg(test)]
mod test;
