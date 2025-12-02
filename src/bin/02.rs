advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<&str> = input.trim().split(",").collect();
    let mut sum: u64 = 0;

    for range in ranges {
        let numbers: Vec<&str> = range.split("-").collect();
        let start: u64 = numbers[0].parse().unwrap();
        let end: u64 = numbers[1].parse().unwrap();

        for n in start..=end {
            let n_stringyfied = n.to_string();
            let n_len = n_stringyfied.len();

            let is_valid_len = n_len % 2 == 0;

            if is_valid_len {
                let pattern_len = n_len / 2;
                let (first, end) = n_stringyfied.split_at(pattern_len);

                if first == end {
                    sum += n;
                }
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<&str> = input.trim().split(",").collect();
    let mut sum: u64 = 0;

    for range in ranges {
        let numbers: Vec<&str> = range.split("-").collect();
        let start: u64 = numbers[0].parse().unwrap();
        let end: u64 = numbers[1].parse().unwrap();

        for n in start..=end {
            let n_stringyfied = n.to_string();
            let n_len = n_stringyfied.len();

            for i in 1..n_len {
                let can_repeat_pattern = n_len % i == 0;

                if !can_repeat_pattern {
                    continue;
                }

                let pattern = &n_stringyfied[..i];
                let repeat_number = n_len / i;

                if pattern.repeat(repeat_number) == n_stringyfied {
                    sum += n;
                    break;
                }
            }
        }
    }

    Some(sum)
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
