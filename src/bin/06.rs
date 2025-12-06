advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    fn create_matrix(input: &str) -> Vec<Vec<&str>> {
        let mut matrix = Vec::new();
        for line in input.lines() {
            let line_array: Vec<&str> = line.trim().split_whitespace().collect();
            matrix.push(line_array.clone());
        }
        matrix
    }

    fn transposition(matrix: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
        let mut transposed_matrix: Vec<Vec<&str>> = Vec::new();

        let num_rows = matrix.len();
        let num_cols = matrix[0].len();

        for x in 0..num_cols {
            let mut new_row: Vec<&str> = Vec::new();
            for y in 0..num_rows {
                new_row.push(matrix[y][x]);
            }
            transposed_matrix.push(new_row);
        }

        transposed_matrix
    }

    let mut matrix = create_matrix(input);
    let operators = matrix.pop().unwrap();
    let matrix = transposition(matrix);
    let mut sum: u64 = 0;

    for (y, row) in matrix.iter().enumerate() {
        let current_operator = operators[y];
        let mut current_number: u64 = 0;
        for x in 0..row.len() - 1 {
            if current_number == 0 {
                current_number = matrix[y][x].parse().unwrap();
            }

            let next_number: u64 = matrix[y][x + 1].parse().unwrap();

            current_number = match current_operator {
                "+" => current_number + next_number,
                "*" => current_number * next_number,
                _ => panic!("Error"),
            };
        }

        sum += current_number;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    fn process_line(line: &str) -> Vec<&str> {
        let chunk_size = line.split_whitespace().map(|s| s.len()).max().unwrap(); // 3 for exemple;
        let mut new_line: Vec<&str> = Vec::new();
        let mut start = 0;

        while start < line.len() {
            let end = (start + chunk_size).min(line.len());
            println!("line: {:?}", new_line);
            new_line.push(&line[start..end]);
            start = end + 1;
        }

        new_line
    }

    fn create_matrix(lines: Vec<&str>) -> Vec<Vec<&str>> {
        let mut matrix = Vec::new();
        for line in lines {
            let new_line = process_line(line);
            matrix.push(new_line);
        }
        matrix
    }

    fn transposition(matrix: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
        let mut transposed_matrix: Vec<Vec<&str>> = Vec::new();

        let num_rows = matrix.len();
        let num_cols = matrix[0].len();

        for x in 0..num_cols {
            let mut new_row: Vec<&str> = Vec::new();
            for y in 0..num_rows {
                new_row.push(matrix[y][x]);
            }
            transposed_matrix.push(new_row);
        }

        transposed_matrix
    }

    fn cephalopod_formatting(matrix: Vec<Vec<&str>>) -> Vec<Vec<String>> {
        let mut new_matrix: Vec<Vec<String>> = Vec::new();
        for y in 0..matrix.len() {
            let row = &matrix[y];
            let mut new_row: Vec<Vec<char>> = vec![Vec::new(); row.len()];

            for x in 0..row.len() {
                let mut chars: Vec<char> = matrix[y][x].chars().collect();
                chars.reverse();
                chars
                    .iter()
                    .enumerate()
                    .for_each(|(index, c)| new_row[index].push(*c));
            }

            let result: Vec<String> = new_row
                .iter()
                .map(|chars| chars.iter().collect::<String>().trim().to_string())
                .collect();

            new_matrix.push(result);
        }

        new_matrix.reverse();
        new_matrix
    }

    let mut input_lines: Vec<&str> = input.lines().collect();
    let operators: &str = input_lines.pop().unwrap();
    let operators: Vec<&str> = operators.split_whitespace().rev().collect();

    let foo = input_lines.iter().map(|line| line.replace(" ", ","));

    let matrix = create_matrix(input_lines);
    let matrix = transposition(matrix);
    let matrix = cephalopod_formatting(matrix);
    let mut sum: u64 = 0;

    for (y, row) in matrix.iter().enumerate() {
        let current_operator = operators[y];
        let mut current_number: u64 = 0;
        for x in 0..row.len() - 1 {
            if current_number == 0 {
                current_number = matrix[y][x].parse().unwrap_or(0);
            }

            let next_number: u64 = matrix[y][x + 1].parse().unwrap_or(0);

            current_number = match current_operator {
                "+" => current_number + next_number,
                "*" => current_number * next_number,
                _ => panic!("Error"),
            };
        }

        sum += current_number;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
