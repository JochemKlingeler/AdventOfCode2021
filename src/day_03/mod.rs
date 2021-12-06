use std::collections::HashMap;
use std::fs;

pub fn part1() -> i32 {
    do_part1(&get_input())
}

fn do_part1(input: &str) -> i32 {
    let mut total_lines = 0;

    // Keep score of the amount of `1`s for each column
    let mut gamma_count = HashMap::new();

    let mut total_columns = 0;
    input.lines().for_each(|s| {
        total_lines += 1;
        let mut i = 1;
        for char in s.chars() {
            let count = gamma_count.entry(i).or_insert(0);
            if '1' == char {
                *count += 1;
            }
            i += 1;
        }
        if 0 == total_columns {
            total_columns = i;
        }
    });

    let mut gamma_binary = String::new();
    let mut epsilon_binary = String::new();
    // Cast to float, as in rust, dividing an integer with another integer returns an integer.
    let total_lines = f64::from(total_lines);
    for i in 1..total_columns {
        let count = gamma_count.get(&i).expect("Expected key to exist");
        // Dereference count
        let is_most_common = (f64::from(*count) / total_lines) >= 0.5;

        if is_most_common {
            gamma_binary.push('1');
            epsilon_binary.push('0');
        } else {
            gamma_binary.push('0');
            epsilon_binary.push('1');
        }
    }

    let gamma_value =
        i32::from_str_radix(&gamma_binary, 2).expect("Expected gamma to be valid binary string!");
    let epsilon_value = i32::from_str_radix(&epsilon_binary, 2)
        .expect("Expected epsilon to be valid binary string!");

    gamma_value * epsilon_value
}

pub fn part2() -> i32 {
    do_part2(&get_input())
}

fn do_part2(input: &str) -> i32 {
    get_oxygen_generator_rating(input) * get_co2_scrubber_rating(input)
}

fn get_oxygen_generator_rating(input: &str) -> i32 {
    let vec_input: Vec<&str> = input.lines().collect();
    let oxygen = recursive_find_through_list(&vec_input, 0, true);
    i32::from_str_radix(oxygen, 2).expect("Can parse binary")
}

fn get_co2_scrubber_rating(input: &str) -> i32 {
    let vec_input: Vec<&str> = input.lines().collect();
    let co2 = recursive_find_through_list(&vec_input, 0, false);
    i32::from_str_radix(co2, 2).expect("Can parse binary")
}

fn recursive_find_through_list<'a>(input: &[&'a str], index: usize, most_common: bool) -> &'a str {
    if input.len() == 1 {
        return input.iter().last().unwrap();
    }
    let mut one_list: Vec<&str> = Vec::new();
    let mut zero_list: Vec<&str> = Vec::new();
    for s in input {
        let char = s.chars().nth(index).unwrap();
        if '1' == char {
            one_list.push(s);
        } else {
            zero_list.push(s);
        }
    }

    let one_count = one_list.len();
    let zero_count = zero_list.len();
    let list_to_use;
    if most_common {
        if one_count >= zero_count {
            list_to_use = one_list;
        } else {
            list_to_use = zero_list;
        }
    } else if one_count >= zero_count {
        list_to_use = zero_list;
    } else {
        list_to_use = one_list;
    }
    recursive_find_through_list(&list_to_use, index + 1, most_common)
}

fn get_input() -> String {
    fs::read_to_string("src/day_03/input.txt").expect("Could not load input file!")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1_demo() {
        assert_eq!(198, do_part1(TEST_INPUT));
    }

    #[test]
    fn part2_demo() {
        assert_eq!(230, do_part2(TEST_INPUT));
    }

    #[test]
    fn part2_oxygen_demo() {
        assert_eq!(23, get_oxygen_generator_rating(TEST_INPUT));
    }

    #[test]
    fn part2_co2_demo() {
        assert_eq!(10, get_co2_scrubber_rating(TEST_INPUT));
    }
}
