use std::fs;

enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn value(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
        }
    }

    fn representation(&self) -> i32 {
        match self {
            RPS::Rock => 0,
            RPS::Paper => 1,
            RPS::Scissors => 2
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win
}

impl Outcome {
    fn value(&self) -> i32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6
        }
    }

    fn representation(&self) -> i32 {
        match self {
            Outcome::Draw => 0,
            Outcome::Win => 1,
            Outcome::Lose => 2
        }
    }
}

#[derive(Debug)]
enum Choice {
    A,
    B,
    C,
    X,
    Y,
    Z
}

impl Choice {
    fn to_rps(&self) -> RPS {
        match self {
            Choice::A => RPS::Rock,
            Choice::B => RPS::Paper,
            Choice::C => RPS::Scissors,
            Choice::X => RPS::Rock,
            Choice::Y => RPS::Paper,
            Choice::Z => RPS::Scissors
        }
    }

    fn to_outcome(&self) -> Outcome {
        match self {
            Choice::X => Outcome::Lose,
            Choice::Y => Outcome::Draw,
            Choice::Z => Outcome::Win,
            c => panic!("Choice::to_outcome: {:?} does not correspond to an Outcome", c)
        }
    }
}

// WISHLIST
// - input -> Choice
// - Choice -> RPS
// - Choice -> Outcome
// - RPS, RPS -> Outcome
// - RPS, Outcome -> RPS

fn parse_input(file_path: &str) -> Vec<(Choice, Choice)> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    lines.map(|line| {
        let v: Vec<&str> = line.split_ascii_whitespace().collect();
        assert_eq!(v.len(), 2);
        let opp_choice = to_choice(v[0]);
        let your_choice = to_choice(v[1]);

        (opp_choice, your_choice)
    }).collect()
}

// Interprets the first value of each line as the opponent's move,
// and the second value as the outcome.
fn input_to_part_2_data(input: Vec<(Choice, Choice)>) -> Vec<(RPS, Outcome)> {
    input.iter().map(|row| {
        let (val1, val2) = row;
        (val1.to_rps(), val2.to_outcome())
    }).collect()
}

fn to_choice(s: &str) -> Choice {
    match s {
        "A" => Choice::A,
        "B" => Choice::B,
        "C" => Choice::C,
        "X" => Choice::X,
        "Y" => Choice::Y,
        "Z" => Choice::Z,
        other => panic!("to_choice: {:?} does not correspond to a Choice", other)
    }
}

fn rps_from_representation(v: i32) -> RPS {
    match v {
        0 => RPS::Rock,
        1 => RPS::Paper,
        2 => RPS::Scissors,
        other => panic!("rps_from_value: {:?} does not correspond to a RPS", other)
    }
}

//    r = (p - o)  mod 3
// => (r + o) = p  mod 3
fn calculate_your_move(opponent_move: &RPS, outcome: &Outcome) -> RPS {
    let r = outcome.representation();
    let o = opponent_move.representation();

    let p = (r + o).rem_euclid(3);

    rps_from_representation(p)
}

fn round_value(your_move: &RPS, outcome: &Outcome) -> i32 {
    your_move.value() + outcome.value()
}


fn part_2_data_total_value(data: Vec<(RPS, Outcome)>) -> i32 {
    data.iter().fold(0, |acc, datum| {
        let (opponent_move, outcome) = datum;
        let your_move = calculate_your_move(opponent_move, outcome);
        acc + round_value(&your_move, outcome)
    })
}

pub fn run_part_2(file_path: &str) {
    let input = parse_input(file_path);
    let data = input_to_part_2_data(input);
    println!("Part 2: {:?}", part_2_data_total_value(data));
}
