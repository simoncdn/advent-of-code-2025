advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut poped_chars = chars.clone();
        poped_chars.pop();

        let first_number = poped_chars.iter().max().unwrap();
        let first_number_index = chars.iter().position(|c| c == first_number).unwrap();
        let mut second_number = 0;

        for i in (first_number_index + 1)..chars.len() {
            let next = chars[i].to_digit(10).unwrap();
            if second_number < next {
                second_number = next
            }
        }

        let final_number: u32 = format!("{}{}", first_number, second_number)
            .parse()
            .unwrap();
        sum += final_number
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    fn recursive(chars: &mut Vec<(usize, char)>, indexes: &mut Vec<usize>) {
        let (index, (char_index, max)) = chars
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.1.cmp(&b.1.1))
            .unwrap();

        println!("char_index: {} | max: {}", char_index, max);

        indexes.push(*char_index);
        chars.remove(index);
    }

    for line in input.lines() {
        let based: Vec<char> = line.chars().collect();
        let mut chars: Vec<(usize, char)> = line.chars().enumerate().collect();
        let mut indexes: Vec<usize> = vec![];

        for _ in 0..12 {
            recursive(&mut chars, &mut indexes);
        }

        println!("i: {:?}", indexes);
        indexes.sort();
        println!("r: {:?}", indexes);

        let result: String = indexes.iter().map(|&i| based[i]).collect();
        println!("{:?}", result);
        sum += result.parse::<u64>().unwrap()
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
