use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::ops::ControlFlow;

fn find_start_of_packet_marker(file_path: &str) -> usize {
    // TODO: Try this again with buffered reading
    let contents = fs::read_to_string(file_path).unwrap();
    let mut last_4: VecDeque<char> = VecDeque::new();

    let result =
        contents.chars().enumerate().try_for_each(|(i, c)| {
            if last_4.len() < 4 {
                last_4.push_back(c);
                ControlFlow::Continue(())
            } else {
                last_4.pop_front();
                last_4.push_back(c);
                let unique: HashSet<char> = HashSet::from_iter(last_4.clone());

                if unique.len() == 4 {
                    ControlFlow::Break(i + 1)
                } else {
                    ControlFlow::Continue(())
                }
            }
        });

    match result {
        ControlFlow::Break(n) => n,
        ControlFlow::Continue(_) => panic!("didn't find a start of packet marker")
    }
}

pub fn run_part_1(file_path: &str) {
    println!("Day 6 part 1: {:?}", find_start_of_packet_marker(file_path));
}
