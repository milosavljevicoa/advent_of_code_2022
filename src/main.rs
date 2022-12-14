pub mod day_one;
pub mod day_two;
pub mod day_three;
pub mod day_four;

fn main() {
    // let day_one_input = Some("inputs/day_one/part_one.txt");
    // println!("Result of day one, part one: {}", day_one::part_one(day_one_input));
    // println!("Result of day one, part one: {:?}", day_one::part_one_v2(false));
    // println!("Result of day one, part two: {}", day_one::part_two(day_one_input));
    // println!("Result of day one, part two: {:?}", day_one::part_two_v2(false));

    // println!("Result of day two, part one: {:?}", day_two::part_one(true));
    // println!("Result of day two, part one: {:?}", day_two::part_one(false));
    // println!("Result of day two, part two: {:?}", day_two::part_two(true));
    // println!("Result of day two, part two: {:?}", day_two::part_two(false));

    // println!("Result of day three, part one: {:?}", day_three::part_one(true));
    // println!("Result of day three, part one: {:?}", day_three::part_one(false));
    // println!("Result of day three, part two: {:?}", day_three::part_two(true));
    // println!("Result of day three, part two: {:?}", day_three::part_two(false));
    
    println!("Result of day four, part one: {:?}", day_four::part_one(true));
    println!("Result of day four, part one: {:?}", day_four::part_one(false));
    println!("Result of day four, part two: {:?}", day_four::part_two(true));
    // println!("Result of day four, part two: {:?}", day_four::part_two(false));
}
