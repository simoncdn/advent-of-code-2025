advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let white_space = lines.iter().position(|&l| l == "").unwrap();
    let (ranges, ingredients) = lines.split_at(white_space);
    let ranges: Vec<(u64, u64)> = ranges
        .iter()
        .map(|range| {
            let parts: Vec<&str> = range.split("-").collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();

    let mut fresh_ingredients: u64 = 0;
    for ingredient in ingredients.iter() {
        if *ingredient == "" {
            continue;
        }

        let ingredient_number: u64 = ingredient.parse().unwrap();
        let find = ranges
            .iter()
            .find(|(min, max)| ingredient_number >= *min && ingredient_number <= *max);

        if find.is_some() {
            fresh_ingredients += 1
        }
    }

    Some(fresh_ingredients)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let white_space = lines.iter().position(|&l| l == "").unwrap();
    let (ranges, _) = lines.split_at(white_space);

    let mut ranges: Vec<(u64, u64)> = ranges
        .iter()
        .map(|range| {
            let parts: Vec<&str> = range.split("-").collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged: Vec<(u64, u64)> = Vec::new();

    for (min, max) in ranges.iter() {
        if let Some(last) = merged.last_mut() {
            if *min <= last.1 + 1 {
                last.1 = last.1.max(*max);
            } else {
                merged.push((*min, *max));
            }
        } else {
            merged.push((*min, *max));
        }
    }

    let total: u64 = merged.iter().map(|(min, max)| max - min + 1).sum();

    Some(total)
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
