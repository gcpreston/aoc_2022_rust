use std::fs;

fn parse_input(file_path: &str) -> Vec<[u32; 4]> {
    let contents = fs::read_to_string(file_path).unwrap();

    contents.lines().map(|line| {
        let (assignment_1, assignment_2) = line.split_once(',').unwrap();
        let (elf_1_start, elf_1_end) = assignment_1.split_once('-').unwrap();
        let (elf_2_start, elf_2_end) = assignment_2.split_once('-').unwrap();

        [
            elf_1_start.parse::<u32>().unwrap(),
            elf_1_end.parse::<u32>().unwrap(),
            elf_2_start.parse::<u32>().unwrap(),
            elf_2_end.parse::<u32>().unwrap()
        ]
    }).collect()
}

fn fully_overlapping(assignment: [u32; 4]) -> bool {
    let elf_1_start = assignment[0];
    let elf_1_end = assignment[1];
    let elf_2_start = assignment[2];
    let elf_2_end = assignment[3];

    (elf_1_start <= elf_2_start && elf_1_end >= elf_2_end) || (elf_2_start <= elf_1_start && elf_2_end >= elf_1_end)
}

// 2-4,6-8
// 5-7,7-9
// 7-9,5-7
fn any_overlap(assignment: [u32; 4]) -> bool {
    let elf_1_start = assignment[0];
    let elf_1_end = assignment[1];
    let elf_2_start = assignment[2];
    let elf_2_end = assignment[3];

    (elf_1_end >= elf_2_start && elf_1_start <= elf_2_end) || (elf_2_end >= elf_1_start && elf_2_start <= elf_1_end)
}

pub fn run_part_1(file_path: &str) {
    let data = parse_input(file_path);
    let full_overlap_count = data.iter().fold(0, |total, assignment| {
        if fully_overlapping(*assignment) {
            total + 1
        } else {
            total
        }
    });

    println!("Day 4 part 1: {full_overlap_count}");
}

pub fn run_part_2(file_path: &str) {
    let data = parse_input(file_path);
    let any_overlap_count = data.iter().fold(0, |total, assignment| {
        if any_overlap(*assignment) {
            total + 1
        } else {
            total
        }
    });

    println!("Day 4 part 2: {any_overlap_count}");
}
