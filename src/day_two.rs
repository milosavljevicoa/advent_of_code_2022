const FILE_DATA: &str = include_str!("../inputs/day_two/part_one.txt");
const FILE_TEST_DATA: &str = include_str!("../inputs/day_two/part_one_test.txt");

#[derive(PartialEq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scisor,
}


fn get_outcome_points(oponent: &Choice, myself: &Choice) -> u32 {
    match (oponent, myself) {
        // I have won
        (Choice::Rock, Choice::Paper) => 6,
        (Choice::Scisor, Choice::Rock) => 6,
        (Choice::Paper, Choice::Scisor) => 6,
        // draw
        (op_choice, my_choice) if op_choice == my_choice => 3,
        // I have lost
        _ => 0,
    }
}

fn get_choice_points(choice: &Choice) -> u32 {
    match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scisor => 3,
    }
}

fn convert(data: &'static str) -> Vec<&str> {
    data.split("\n").filter(|s| !s.is_empty()).collect()
}

fn convert_line_to_choice(line: &str) -> (Choice, Choice) {
    let mut round_play: Vec<Choice> = line
        .split(' ')
        .into_iter()
        .map(|choice| match choice {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scisor,
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scisor,
            non_valid => panic!("Oponent choice {} not found", non_valid),
        })
        .collect();
    (round_play.remove(0), round_play.remove(0))
}

pub fn part_one(use_test_data: bool) -> u32 {
    let data: &'static str = if use_test_data {
        FILE_TEST_DATA
    } else {
        FILE_DATA
    };

    convert(data)
        .into_iter()
        .map(convert_line_to_choice)
        .map(|(oponent, myself)| get_choice_points(&myself) + get_outcome_points(&oponent, &myself))
        .sum()
}

enum Outcome {
    Win,
    Lose,
    Draw
}

fn convert_line_to_choice_coded(line: &str) -> (Choice, Choice) {
    let mut code: Vec<&str> = line.split(' ').collect();

    let (opponent, myself) = (code.remove(0), code.remove(0));
    let opponent_choice = match opponent {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scisor,
        non_valid => panic!("Oponent choice {} not found", non_valid),
    };

    let my_outcome = match myself {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        non_valid => panic!("Oponent choice {} not found", non_valid),
    };

    let my_chose = get_my_choice(&my_outcome, &opponent_choice);

    (opponent_choice, my_chose)
}

fn get_my_choice(outcome: &Outcome, opponent: &Choice) -> Choice {
    match (outcome, opponent) {
        (Outcome::Win, Choice::Rock) => Choice::Paper,
        (Outcome::Win, Choice::Paper) => Choice::Scisor,
        (Outcome::Win, Choice::Scisor) => Choice::Rock,

        (Outcome::Draw, opponent) => *opponent,

        (Outcome::Lose, Choice::Rock) => Choice::Scisor,
        (Outcome::Lose, Choice::Paper) => Choice::Rock,
        (Outcome::Lose, Choice::Scisor) => Choice::Paper,
    }
}

pub fn part_two(use_test_data: bool) -> u32 {
    let data: &'static str = if use_test_data {
        FILE_TEST_DATA
    } else {
        FILE_DATA
    };

    convert(data)
        .into_iter()
        .map(convert_line_to_choice_coded)
        .map(|(oponent, myself)| get_choice_points(&myself) + get_outcome_points(&oponent, &myself))
        .sum()
}
