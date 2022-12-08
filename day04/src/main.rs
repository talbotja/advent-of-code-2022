use std::fs;
use std::ops::Range;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Failed to read input");

    let pairs = contents.lines().map(parse_pair);

    let p1: usize = pairs.clone().filter(overlap_fully).count();

    let p2: usize = pairs.filter(overlap_at_all).count();

    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

fn parse_pair(line: &str) -> (Range<i32>, Range<i32>) {
  let mut r = line.split(',')
    .map(|s| {
      let mut nums = s.split('-').map(|num| num.parse::<i32>().unwrap());
      nums.next().unwrap()..(1 + nums.next().unwrap())
    });

  (r.next().unwrap(), r.next().unwrap())
}

fn overlap_fully(pair: &(Range<i32>, Range<i32>)) -> bool {
  let (a, b) = pair;
  (a.contains(&b.start) && a.contains(&(b.end - 1))) || (b.contains(&a.start) && b.contains(&(a.end - 1)))
}

fn overlap_at_all(pair: &(Range<i32>, Range<i32>)) -> bool {
  let (a, b) = pair;
  a.contains(&b.start) || a.contains(&(b.end - 1)) || b.contains(&a.start) || b.contains(&(a.end - 1))
}
