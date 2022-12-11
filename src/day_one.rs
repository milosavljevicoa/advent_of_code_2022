use std::fs;

static ELF_LIST: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

pub fn part_one(file_path: Option<&str>) -> u32 {
    let mut max: u32 = 0;
    let mut tmp: u32 = 0;

    let non: Option<i32> = None;
    let som = Some(1);

    let input = file_path
        .map(|path| fs::read_to_string(path).unwrap())
        .unwrap_or(String::from(ELF_LIST));

    for line in input.lines() {
        if line.is_empty() {
            if tmp > max {
                max = tmp;
            }
            tmp = 0;
            continue;
        }

        let calories: u32 = line.parse().unwrap();
        tmp = tmp + calories;
    }

    max
}

pub fn part_two(file_path: Option<&str>) -> u32 {
    let input = file_path
        .map(move |str| fs::read_to_string(str).unwrap())
        .unwrap_or(String::from(ELF_LIST));

    let mut max_calories: Vec<u32> = vec![0, 0, 0];
    let mut calories_for_elf: u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            add_element_to_desc_sorted_list(&mut max_calories, calories_for_elf);
            calories_for_elf = 0;
            continue;
        }

        let calories: u32 = line.parse().unwrap();
        calories_for_elf = calories_for_elf + calories;
    }

    add_element_to_desc_sorted_list(&mut max_calories, calories_for_elf);

    max_calories.into_iter().sum()
}

fn add_element_to_desc_sorted_list(desc_list: &mut Vec<u32>, element_to_add: u32) {
    let mut element_to_add = element_to_add;
    for list_element in desc_list {
        if *list_element < element_to_add {
            let tmp = element_to_add;
            element_to_add = *list_element;
            *list_element = tmp;
        }
    }
}

// ref: https://github.com/Crazytieguy/advent-of-code/blob/master/2022/src/bin/day1/main.rs
const FILE_DATA: &str = include_str!("../inputs/day_one/part_one.txt");
const FILE_TEST_DATA: &str = include_str!("../inputs/day_one/part_one_test.txt");

fn parse(data: &'static str) -> Vec<u32> {
    data.split("\n\n")
        .into_iter()
        .map(|elf_calories| -> u32 {
            elf_calories
                .lines()
                .into_iter()
                .map(|calories| -> u32 { calories.parse().unwrap() })
                .sum()
        })
        .collect()
}

pub fn part_one_v2(test: bool) -> u32 {
    let data = if test { FILE_TEST_DATA } else { FILE_DATA };
    parse(data)
        .into_iter()
        .max()
        .unwrap()
}

pub fn part_two_v2(test: bool) -> u32 {
    let data = if test { FILE_TEST_DATA } else { FILE_DATA };
    let mut inventory = parse(data);
    inventory.sort_by_key(|p| std::cmp::Reverse(*p));
    inventory
        .into_iter()
        .take(3)
        .sum()
}
