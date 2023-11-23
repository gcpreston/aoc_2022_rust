// Wishlist
// - get_item_priority
// - rucksack_common_item

use std::fs;
use std::collections::HashSet;

fn rucksack_common_item(rucksack: &str) -> String {
  let compartments_size = rucksack.len() / 2;
  let (compartment_1, compartment_2) = rucksack.split_at(compartments_size);

  let compartment_1_set: HashSet<char> = compartment_1.chars().collect();
  let compartment_2_set: HashSet<char> = compartment_2.chars().collect();
  let mut intersection = compartment_1_set.intersection(&compartment_2_set);

  let mut ret = String::new();

  match intersection.next() {
    Some(item) => ret.push(*item),
    None => panic!("no intersection was found")
  };

  ret
}

fn group_common_item(group: &[&str]) -> String {
  let rucksack_2_set: HashSet<char> = group[1].chars().collect();
  let rucksack_3_set: HashSet<char> = group[2].chars().collect();

  let mut common_item = group[0].chars().filter(|c| rucksack_2_set.contains(c) && rucksack_3_set.contains(c));

  let mut ret = String::new();

  match common_item.next() {
    Some(item) => ret.push(item),
    None => panic!("no intersection was found")
  };

  ret
}

fn get_item_priority(item: &char) -> u32 {
  let is_upper = item.is_uppercase();
  let standardized = item.to_lowercase().next().unwrap();

  let mut val = (standardized as u32) - 96; // 'a' = 97

  if is_upper {
    val += 26;
  }

  val
}

pub fn run_part_1(file_path: &str) {
  let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
  let lines = contents.lines();

  let result = lines.fold(0, |total, rucksack| {
    let common_item = rucksack_common_item(rucksack);
    let item_as_char = common_item.chars().next().unwrap();
    total + get_item_priority(&item_as_char)
  });

  println!("Day 3 part 1: {result}");
}

pub fn run_part_2(file_path: &str) {
  let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
  let lines: Vec<&str> = contents.lines().collect();

  let result = lines.chunks(3).fold(0, |total, group| {
    let common_item = group_common_item(group);
    let item_as_char = common_item.chars().next().unwrap();
    total + get_item_priority(&item_as_char)
  });

  println!("Day 3 part 2: {result}");
}
