use std::fs;
use regex::Regex;

fn apply_move_part_1(stacks: &mut [Vec<char>; 9], count: u32, from: usize, to: usize) {
    for _ in 0..count {
        let c = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(c);
    }
}

fn apply_move_part_2(stacks: &mut [Vec<char>; 9], count: usize, from: usize, to: usize) {
    /* TODO: Still would like to do this in not O(n)
    let from_stack = stacks[from - 1];
    let mut to_stack  = stacks[to - 1];

    let slice_to_move = &from_stack[from_stack.len() - count..];
    to_stack.extend_from_slice(slice_to_move);
    */

    let mut temp: Vec<char> = Vec::new();

    for _ in 0..count {
        let c = stacks[from - 1].pop().unwrap();
        temp.push(c);
    }

    for _ in 0..count {
        stacks[to - 1].push(temp.pop().unwrap());
    }
}

/*
fn take_slice(stacks: &mut [Vec<char>; 9], count: usize, from: usize) -> Vec<char> {
    let mut from_stack = &stacks[from - 1];
    let keep_slice = &from_stack[..from_stack.len() - count];
    let take_slice = &from_stack[from_stack.len() - count..];

    stacks[from - 1] = keep_slice.to_vec();
    take_slice.to_vec()
}
 */

pub fn run_part_1(file_path: &str) {
    // let mut stacks: [Vec<char>; 3] = [vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

    let mut stacks: [Vec<char>; 9] = [
        vec!['W', 'R', 'F'],
        vec!['T', 'H', 'M', 'C', 'D', 'V', 'W', 'P'],
        vec!['P', 'M', 'Z', 'N', 'L'],
        vec!['J', 'C', 'H', 'R'],
        vec!['C', 'P', 'G', 'H', 'Q', 'T', 'B'],
        vec!['G', 'C', 'W', 'L', 'F', 'Z'],
        vec!['W', 'V', 'L', 'Q', 'Z', 'J', 'G', 'C'],
        vec!['P', 'N', 'R', 'F', 'W', 'T', 'V', 'C'],
        vec!['J', 'W', 'H', 'G', 'R', 'S', 'V']
    ];

    let contents = fs::read_to_string(file_path).unwrap();
    let (_layout, instructions) = contents.split_once("\n\n").unwrap();

    let instruction_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    instructions.lines().for_each(|instruction| {
        let captures = instruction_re.captures(instruction).unwrap();
        let count: usize = captures[1].parse().unwrap();
        let from: usize = captures[2].parse().unwrap();
        let to: usize = captures[3].parse().unwrap();

        let from_stack = &stacks[from - 1];
        let to_stack  = &mut stacks[to - 1];

        let slice_to_move = &from_stack[from_stack.len() - count..];
        to_stack.extend_from_slice(slice_to_move);
    });

    let result: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();

    println!("Day 5 part 2: {result}");
}
