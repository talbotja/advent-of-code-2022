use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Failed to read input");

    let rounds =
      contents
        .trim_end_matches("\n")
        .split("\n");

    let p1: i32 = rounds.clone().map(parse_round_p1).map(calculate_score).sum();

    let p2: i32 = rounds.map(parse_round_p2).map(calculate_score).sum();

    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

#[derive(Clone, Copy)]
enum Shape {
  Rock = 0,
  Paper = 1,
  Scissors = 2,
}

impl Shape {
  fn from(i: u8) -> Shape {
    match i {
      0 => Shape::Rock,
      1 => Shape::Paper,
      2 => Shape::Scissors,
      _ => panic!("invalid shape: {}", i),
    }
  }
}

struct Round {
  i_play: Shape,
  opponent_plays: Shape,
}

fn calculate_score(round: Round) -> i32 {
  let shape_score = round.i_play as u8 + 1;
  let win_score = 3 * ((4 + round.i_play as u8 - round.opponent_plays as u8) % 3);
  (shape_score + win_score).into()
}

fn parse_round_p1(s: &str) -> Round {
  let opponent_plays = Shape::from(s.bytes().nth(0).unwrap() - b'A');
  let i_play = Shape::from(s.bytes().nth(2).unwrap() - b'X');
  return Round {
    i_play,
    opponent_plays,
  };
}

fn parse_round_p2(s: &str) -> Round {
  let opponent_plays = Shape::from(s.bytes().nth(0).unwrap() - b'A');
  let target_result: u8 = s.bytes().nth(2).unwrap() - b'X'; // 0=loss, 1=draw, 2=win
  let i_play = Shape::from((2 + opponent_plays as u8 + target_result) % 3);
  return Round {
    i_play,
    opponent_plays,
  };
}
