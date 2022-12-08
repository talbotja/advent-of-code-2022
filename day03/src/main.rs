#![feature(iter_array_chunks)]
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Failed to read input");

    let bags = contents.lines();

    let p1: i32 = bags.clone().map(find_duplicate_item).sum();

    let p2: i32 =
      bags
        .array_chunks()
        .map(find_shared_item)
        .sum();

    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

fn priority(item: u8) -> i32 {
  if item <= b'Z' {
    (27 + item - b'A').into()
  } else {
    (1 + item - b'a').into()
  }
}

fn find_shared_item(three_bags: [&str; 3]) -> i32 {
  let dupe =
    three_bags
      .map(|bag| bag.bytes().collect())
      .into_iter()
      .reduce(|a: HashSet<u8>, b: HashSet<u8>| a.intersection(&b).map(|i| *i).collect())
      .unwrap();

  priority(*dupe.iter().next().unwrap())
}

fn find_duplicate_item(bag: &str) -> i32 {
  let len = bag.len();
  let comp_a: HashSet<u8> = bag.get(0..len/2).unwrap().bytes().collect();
  let comp_b: HashSet<u8> = bag.get(len/2..len).unwrap().bytes().collect();
  let mut dupe = comp_a.intersection(&comp_b);
  priority(*dupe.next().unwrap())
}
