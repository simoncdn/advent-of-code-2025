advent_of_code::solution!(4);

//(Y-1, X-1)(Y-1, X)(Y-1, X+1)
//(Y,   X-1)        (Y,   X+1)
//(Y+1, X-1)(Y+1, X)(Y+1, X+1)

pub fn part_one(input: &str) -> Option<u64> {
    fn create_matrix(input: &str) -> Vec<Vec<char>> {
        let mut matrix = Vec::new();
        for line in input.lines() {
            let chars: Vec<char> = line.chars().collect();
            matrix.push(chars.clone());
        }
        matrix
    }

    const ROLL: char = '@';
    let mut sum: u64 = 0;
    let matrix = create_matrix(input);

    for (y, row) in matrix.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != ROLL {
                continue;
            }

            let mut positions: Vec<(i64, i64)> = vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];

            if y == 0 {
                positions = positions.into_iter().filter(|a| a.0 != -1).collect();
            } else if y == matrix.len() - 1 {
                positions = positions.into_iter().filter(|a| a.0 != 1).collect();
            }

            if x == 0 {
                positions = positions.into_iter().filter(|a| a.1 != -1).collect();
            } else if x == matrix[0].len() - 1 {
                positions = positions.into_iter().filter(|a| a.1 != 1).collect();
            }

            let count = positions
                .iter()
                .filter(|(dy, dx)| {
                    let ny = (y as i64 + dy) as usize;
                    let nx = (x as i64 + dx) as usize;
                    matrix[ny][nx] == ROLL
                })
                .count();

            if count < 4 {
                sum += 1
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    fn create_matrix(input: &str) -> Vec<Vec<char>> {
        let mut matrix = Vec::new();
        for line in input.lines() {
            let chars: Vec<char> = line.chars().collect();
            matrix.push(chars.clone());
        }
        matrix
    }

    const ROLL: char = '@';
    const EMPTY: char = '.';

    let mut sum: u64 = 0;
    let mut matrix = create_matrix(input);

    loop {
        let mut removed_count: u64 = 0;
        for (y, row) in matrix.clone().iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell != ROLL {
                    continue;
                }

                let mut positions: Vec<(i64, i64)> = vec![
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];

                if y == 0 {
                    positions = positions.into_iter().filter(|a| a.0 != -1).collect();
                } else if y == matrix.len() - 1 {
                    positions = positions.into_iter().filter(|a| a.0 != 1).collect();
                }

                if x == 0 {
                    positions = positions.into_iter().filter(|a| a.1 != -1).collect();
                } else if x == matrix[0].len() - 1 {
                    positions = positions.into_iter().filter(|a| a.1 != 1).collect();
                }

                let count = positions
                    .iter()
                    .filter(|(dy, dx)| {
                        let ny = (y as i64 + dy) as usize;
                        let nx = (x as i64 + dx) as usize;
                        matrix[ny][nx] == ROLL
                    })
                    .count();

                if count < 4 {
                    removed_count += 1;
                    sum += 1;
                    matrix[y][x] = EMPTY;
                }
            }
        }
        if removed_count == 0 {
            break;
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
