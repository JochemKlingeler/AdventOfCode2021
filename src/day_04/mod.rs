use std::collections::HashMap;
use std::fs;

pub fn part1() -> i32 {
    do_part1(&get_input())
}

fn do_part1(input: &str) -> i32 {
    1
}

struct BingoCard {
    pub card: HashMap<(i32, i32), (i32, bool)>,
}

impl BingoCard {
    pub fn from_str(input: &str) -> BingoCard {
        let mut card: HashMap<(i32, i32), (i32, bool)> = HashMap::new();
        let mut x = 0;
        input.lines().for_each(|s| {
            let mut y = 0;
            s.split_whitespace().for_each(|c| {
                card.insert((x, y), (c.parse().unwrap(), false));
            });
            x += 1;
        });
        BingoCard { card }
    }

    fn find_key_for_value(&self, value_to_find: i32) -> Option<(i32, i32)> {
        for ((x, y), (value, _isset)) in &self.card {
            if value == &value_to_find {
                return Some((*x, *y));
            }
        }
        None
    }

    pub fn set_bingo_value(mut self, bingo_value: i32) {
        let key = self.find_key_for_value(bingo_value);

        return match key {
            None => (),
            Some((x, y)) => {
                if let Some((_card_value, isset)) = self.card.get_mut(&(x, y)) {
                    *isset = true;
                }
            }
        };
    }
}

fn get_input() -> String {
    fs::read_to_string("src/day_04/input.txt").expect("Could not load input file!")
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part1_demo() {
        assert_eq!(4512, do_part1(TEST_INPUT));
    }
}
