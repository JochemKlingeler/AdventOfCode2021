use std::fs;

pub fn part1() -> i32 {
    do_part1(&get_input())
}

fn do_part1(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let lines = input.lines();
    lines.for_each(|s| {
        let mut split = s.splitn(2, ' ');
        let (direction, unit) = (
            split
                .next()
                .expect("Expected to be able to parse direction"),
            split
                .next()
                .expect("Expected to be able to parse unit")
                .parse::<i32>()
                .expect("Expected to be able to cast unit"),
        );
        match direction {
            "forward" => x += unit,
            "down" => y += unit,
            "up" => y -= unit,
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
}
