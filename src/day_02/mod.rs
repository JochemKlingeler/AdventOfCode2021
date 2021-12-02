use std::fs;

pub fn part1() -> i32 {
    do_part1(&get_input())
}

fn do_part1(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;
    input.lines().for_each(|s| {
        let (direction, unit) = s.split_once(' ').expect("Can split by space");
        let unit: i32 = unit.parse().expect("Can parse unit to int");
        match direction {
            "forward" => x += unit,
            "down" => y += unit,
            "up" => y -= unit,
            _ => panic!("Invalid direction: {}", direction),
        }
    });
    x * y
}

pub fn part2() -> i32 {
    do_part2(&get_input())
}

fn do_part2(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    input.lines().for_each(|s| {
        let (direction, unit) = s.split_once(' ').expect("Can split by space");
        let unit: i32 = unit.parse().expect("Can parse unit to int");
        match direction {
            "forward" => {
                x += unit;
                y += unit * aim
            }
            "down" => aim += unit,
            "up" => aim -= unit,
            _ => panic!("Invalid direction: {}", direction),
        }
    });
    x * y
}

fn get_input() -> String {
    fs::read_to_string("src/day_02/input.txt").expect("Could not load input file!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_demo() {
        assert_eq!(
            150,
            do_part1(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            )
        )
    }

    #[test]
    fn part2_demo() {
        assert_eq!(
            900,
            do_part2(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            )
        )
    }
}
