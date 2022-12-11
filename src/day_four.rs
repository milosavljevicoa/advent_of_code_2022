use std::collections::HashMap;

use advent_of_code_2022::utils;
use array_tool::vec::Intersect;

const FILE_DATA: &str = include_str!("../inputs/day_four/part_one.txt");
const FILE_TEST_DATA: &str = include_str!("../inputs/day_four/part_one_test.txt");

#[derive(Debug, PartialEq)]
struct ElfsCleaningRange {
    starting_section: u32,
    ending_section: u32,
}

impl ElfsCleaningRange {
    fn new(starting_section: u32, ending_section: u32) -> Self {
        Self {
            starting_section,
            ending_section,
        }
    }

    fn has_overlaping_sections(&self, other: &ElfsCleaningRange) -> bool {
        self.starting_section <= other.starting_section
            && self.ending_section >= other.starting_section
    }
}

// fn convert(file: &str) -> Vec<&str> {
//     file.split("\n").filter(|s| !s.is_empty()).collect()
// }

fn parse_line(line: &str) -> (ElfsCleaningRange, ElfsCleaningRange) {
    let mut cleaning_range = line
        .split(",")
        .map(|section_range: &str| {
            section_range
                .split('-')
                .map(|section| section.parse::<u32>().unwrap())
        })
        .map(|section_ids| {
            let nums = section_ids.collect::<Vec<u32>>();
            ElfsCleaningRange::new(nums[0], nums[1])
        })
        .collect::<Vec<ElfsCleaningRange>>();

    let (elf2, elf1) = (cleaning_range.pop().unwrap(), cleaning_range.pop().unwrap());
    (elf1, elf2)
}

fn some_sections_matches(cleaning_range: &(ElfsCleaningRange, ElfsCleaningRange)) -> bool {
    let elfs_section_one = &cleaning_range.0;
    let elfs_section_two = &cleaning_range.1;

    elfs_section_one.has_overlaping_sections(elfs_section_two)
        || elfs_section_two.has_overlaping_sections(elfs_section_one)
}

pub fn part_one(test_data: bool) -> u32 {
    let data = if test_data { FILE_TEST_DATA } else { FILE_DATA };

    utils::convert(data)
        .into_iter()
        .map(parse_line)
        .map(|(elf_one, elf_two)| elf_one.get_overlaping_sections(&elf_two))
        .filter(|overlaping_sections| !overlaping_sections.is_empty())
        .count() as u32
}

pub fn part_two(test_data: bool) -> u32 {
    let data = if test_data { FILE_TEST_DATA } else { FILE_DATA };

    // utils::convert(data)
    //     .into_iter()
    //     .map(parse_line)
    //     .map(|(elf_one, elf_two)| elf_one.get_overlaping_sections(&elf_two))
    //     .filter(|overlaping_sections| !overlaping_sections.is_empty())
    // .flatten()
    // .collect::<HashSet<u32>>()
    // .len() as u32
    let test = utils::convert(data)
        .into_iter()
        .map(parse_line)
        .map(|(elf_one, elf_two)| elf_one.get_overlaping_sections(&elf_two))
        .filter(|overlaping_sections| !overlaping_sections.is_empty())
        .flatten()
        .collect::<Vec<u32>>();

    get_duplicates_count(&test)
}

fn get_duplicates_count(vec: &Vec<u32>) -> u32 {
    let mut num_count: HashMap<&u32, u32> = HashMap::new();

    for num in vec {
        *num_count.entry(num).or_insert(0) += 1;
    }

    num_count.into_iter().filter(|(_, v)| *v > 1).count() as u32
}

impl ElfsCleaningRange {
    // not working as expected
    fn get_overlaping_sections(&self, other: &ElfsCleaningRange) -> Vec<u32> {
        let first_sections: Vec<u32> = (self.starting_section..=self.ending_section).collect();
        let second_sections: Vec<u32> = (other.starting_section..=other.ending_section).collect();
        let sec = first_sections.intersect(second_sections);
        sec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_duplicates_count_test() {
        let example_one = vec![1,4,7,8,9,1,4,1];
        assert_eq!(2, get_duplicates_count(&example_one));
    }

    #[test]
    fn elfs_1() {
        let example_one = "2-4,6-8";
        let (e1, e2) = &parse_line(example_one);
        assert!(e1.get_overlaping_sections(e2).is_empty());
    }

    #[test]
    fn elfs_2() {
        let example_two = "2-3,4-5";
        let (e1, e2) = &parse_line(example_two);
        assert!(e1.get_overlaping_sections(e2).is_empty());
    }

    #[test]
    fn elfs_3() {
        let example_three = "5-7,7-9";
        let (e1, e2) = &parse_line(example_three);
        assert_eq!(1, e1.get_overlaping_sections(e2).len());
    }

    #[test]
    fn elfs_3_rev() {
        let example_three = "7-9,5-7";
        let (e1, e2) = &parse_line(example_three);
        assert_eq!(1, e1.get_overlaping_sections(e2).len());
    }

    #[test]
    fn elfs_4() {
        let example_four = "2-8,3-7";
        let (e1, e2) = &parse_line(example_four);
        assert_eq!(5, e1.get_overlaping_sections(e2).len());
    }

    #[test]
    fn elfs_5() {
        let example_five = "6-6,4-6";
        let (e1, e2) = &parse_line(example_five);
        assert_eq!(1, e1.get_overlaping_sections(e2).len());
    }

    #[test]
    fn elfs_6() {
        let example_six = "2-6,4-8";
        let (e1, e2) = &parse_line(example_six);
        assert_eq!(3, e1.get_overlaping_sections(e2).len());
    }
}
