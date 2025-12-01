advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut counter = 0;

    for line in input.lines() {
        let direction = &line[0..1];
        let number: i32 = line[1..].parse().unwrap();
        let next_position;

        if direction == "L" {
            next_position = position - number;
        } else {
            next_position = position + number;
        }

        if next_position < 0 {
            position = (100 + next_position) % 100;
        } else if next_position > 99 {
            position = ((99 - (next_position - 1)).abs()) % 100;
        } else {
            position = next_position;
        }

        if position == 0 {
            counter += 1;
        }
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut position: i64 = 50;
    let mut counter: i64 = 0;

    for line in input.lines() {
        let direction = &line[0..1];
        let number: i64 = line[1..].parse().unwrap();

        match direction {
            "R" => {
                counter += (position + number) / 100;
                position = (position + number) % 100;
            }
            "L" => {
                if position > 0 && number >= position {
                    counter += (number - position) / 100 + 1;
                } else if position == 0 && number > 0 {
                    counter += number / 100;
                }
                position = (position - number).rem_euclid(100);
            }
            _ => eprint!("Err"),
        }
    }

    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
