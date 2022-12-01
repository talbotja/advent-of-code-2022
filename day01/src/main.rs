use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Failed to read input");

    let mut elves =
      contents
        .trim_end_matches("\n")
        .split("\n\n")
        .map(|elf|
          elf
            .split("\n")
            .map(|n| n.parse::<i32>().unwrap())
            .sum::<i32>()
        )
        .collect::<Vec<_>>();

    elves.sort_by_key(|e| e * -1);

    let p1 = elves.first().unwrap();

    let p2 = elves.get(..3).unwrap().iter().sum::<i32>();

    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}
