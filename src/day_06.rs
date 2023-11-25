use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::ops::ControlFlow;

const START_OF_PACKET_UNIQUE_COUNT: usize = 4;
const START_OF_MESSAGE_UNIQUE_COUNT: usize = 14;

fn index_of_first_n_unique_chars(file_path: &str, n: usize) -> usize {
    // TODO: Try this again with buffered reading
    let contents = fs::read_to_string(file_path).unwrap();
    let mut queue: VecDeque<char> = VecDeque::new();

    let result =
        contents.chars().enumerate().try_for_each(|(i, c)| {
            if queue.len() < n {
                queue.push_back(c);
                ControlFlow::Continue(())
            } else {
                queue.pop_front();
                queue.push_back(c);
                let unique: HashSet<char> = HashSet::from_iter(queue.clone());

                if unique.len() == n {
                    ControlFlow::Break(i + 1)
                } else {
                    ControlFlow::Continue(())
                }
            }
        });

    match result {
        ControlFlow::Break(n) => n,
        ControlFlow::Continue(_) => panic!("did not find {n} consecutive unique characters")
    }
}

fn find_start_of_packet_marker(file_path: &str) -> usize {
    index_of_first_n_unique_chars(file_path, START_OF_PACKET_UNIQUE_COUNT)
}

fn find_start_of_message_marker(file_path: &str) -> usize {
    index_of_first_n_unique_chars(file_path, START_OF_MESSAGE_UNIQUE_COUNT)
}

pub fn run_part_1(file_path: &str) {
    println!("Day 6 part 1: {:?}", find_start_of_packet_marker(file_path));
}

pub fn run_part_2(file_path: &str) {
    println!("Day 6 part 2: {:?}", find_start_of_message_marker(file_path));
}
