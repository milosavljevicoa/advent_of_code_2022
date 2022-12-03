use std::collections::HashSet;

const FILE_DATA: &str = include_str!("../inputs/day_three/part_one.txt");
const FILE_TEST_DATA: &str = include_str!("../inputs/day_three/part_one_test.txt");

fn get_item_type_priority(char: char) -> u32 {
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    let starting_point = if char.is_ascii_lowercase() {
        ('a' as u32) - 1
    } else {
        ('A' as u32) - 27
    };
    (char as u32) - starting_point
}

fn get_same_item_types(line: &str) -> Vec<char> {
    let lenght = line.len() / 2;
    let first_rucksack = line[0..lenght].chars().collect::<HashSet<char>>();
    let second_rucksack = line[lenght..].chars().collect::<HashSet<char>>();

    first_rucksack
        .intersection(&second_rucksack)
        .map(|char| *char)
        .collect::<Vec<char>>()
}

fn convert(line: &str) -> Vec<&str> {
    line.split("\n").filter(|s| !s.is_empty()).collect()
}

pub fn part_one(use_test_data: bool) -> u32 {
    let data = if use_test_data {
        FILE_TEST_DATA
    } else {
        FILE_DATA
    };
    convert(data)
        .into_iter()
        .map(get_same_item_types)
        .map(|line| line.into_iter().map(get_item_type_priority).max().unwrap())
        .sum()
}

pub fn part_two(use_test_data: bool) -> u32 {
    let data = if use_test_data {
        FILE_TEST_DATA
    } else {
        FILE_DATA
    };
    let mut iter = convert(data)
        .into_iter()
        .map(|str| str.chars().collect::<HashSet<char>>())
        .peekable();

    let mut badges: Vec<char> = Vec::new();

    while iter.peek().is_some() {
        let (elf_one, elf_two, elf_three) = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );

        badges.push(elf_one
            .intersection(&elf_two)
            .map(|char| *char)
            .collect::<HashSet<char>>()
            .intersection(&elf_three)
            .next()
            .unwrap()
            .clone()
        )
    }

    badges.into_iter().map(get_item_type_priority).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, get_item_type_priority('a'));
        assert_eq!(26, get_item_type_priority('z'));
        assert_eq!(27, get_item_type_priority('A'));
        assert_eq!(52, get_item_type_priority('Z'));
    }
}
