#![feature(iter_next_chunk)]
use std::collections::VecDeque;
use std::fs;

fn main() {
    let content = fs::read_to_string("input").expect("Failed to read input");

    let [init_lines, instr_lines] = content.split("\n\n").next_chunk().unwrap();

    let mut init_vec = Vec::from_iter(init_lines.lines());
    init_vec.pop();

    let initial_state = parse_initial_state(init_vec);

    let instructions = instr_lines.lines().map(parse_instruction);

    let p1: String = top_crates(
        instructions
            .clone()
            .fold(initial_state.clone(), process_instruction_p1),
    );

    let p2: String = top_crates(instructions.fold(initial_state, process_instruction_p2));

    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}

fn parse_instruction(line: &str) -> (usize, usize, usize) {
    let mut instruction_chars = line.split(' ');
    let num_crates: usize = instruction_chars.nth(1).unwrap().parse().unwrap();
    let from_stack: usize = instruction_chars.nth(1).unwrap().parse().unwrap();
    let to_stack: usize = instruction_chars.nth(1).unwrap().parse().unwrap();
    (num_crates, from_stack, to_stack)
}

fn process_instruction_p1(
    mut stacks: Vec<VecDeque<char>>,
    instruction: (usize, usize, usize),
) -> Vec<VecDeque<char>> {
    let (num_crates, from_stack, to_stack) = instruction;
    (0..num_crates).for_each(|_| {
        let c = stacks[from_stack - 1].pop_front().unwrap();
        stacks[to_stack - 1].push_front(c);
    });
    stacks
}

fn process_instruction_p2(
    mut stacks: Vec<VecDeque<char>>,
    instruction: (usize, usize, usize),
) -> Vec<VecDeque<char>> {
    let (num_crates, from_stack, to_stack) = instruction;
    let mut cs = VecDeque::from_iter(stacks[from_stack - 1].drain(..num_crates));
    cs.append(&mut stacks[to_stack - 1]);
    stacks[to_stack - 1] = cs;
    stacks
}

fn top_crates(stacks: Vec<VecDeque<char>>) -> String {
    stacks.iter().map(|s| s[0]).collect()
}

fn parse_initial_state(lines: Vec<&str>) -> Vec<VecDeque<char>> {
    let line1 = lines[0];
    let num_stacks = (line1.len() + 1) / 4;
    let mut stacks = vec![VecDeque::new(); num_stacks];
    lines.iter().for_each(|line| {
        stacks.iter_mut().enumerate().for_each(|(idx, stack)| {
            let c: char = line.chars().nth(1 + 4 * idx).unwrap();
            if c != ' ' {
                stack.push_back(c);
            }
        });
    });
    stacks
}
