const INPUT: &str = include_str!("input.txt");
pub(crate) fn main() {
    let height_matrix: Vec<Vec<u8>> = matrixify(INPUT);
    part_one(&height_matrix);
    part_two(&height_matrix);
}

fn part_one(height_matrix: &[Vec<u8>]) {
    let mut visible_count = 0;
    for (row_idx, columns) in height_matrix.iter().enumerate() {
        for (col_idx, height) in columns.iter().enumerate() {
            if columns[..col_idx].iter().all(|h| h < height)
                || columns[(col_idx + 1)..].iter().all(|h| h < height)
                || height_matrix[..row_idx]
                    .iter()
                    .all(|row| row[col_idx] < *height)
                || height_matrix[(row_idx + 1)..]
                    .iter()
                    .all(|row| row[col_idx] < *height)
            {
                visible_count += 1;
                continue;
            }
        }
    }

    println!("visible from outside: {visible_count}");
}

fn part_two(height_matrix: &[Vec<u8>]) {
    let mut max_scenic_score = 0;
    for (row_idx, columns) in height_matrix.iter().enumerate() {
        for (col_idx, height) in columns.iter().enumerate() {
            let mut view_blocked = false;

            let horizontal = move |count: i32, h: &u8| {
                if view_blocked {
                    count
                } else {
                    view_blocked = h >= height;
                    count + 1
                }
            };

            let vertical = move |count: i32, row: &Vec<u8>| {
                if view_blocked {
                    count
                } else {
                    view_blocked = row[col_idx] >= *height;
                    count + 1
                }
            };

            let view_left = columns[..col_idx].iter().rev().fold(0, horizontal);
            let view_right = columns[(col_idx + 1)..].iter().fold(0, horizontal);
            let view_up = height_matrix[..row_idx].iter().rev().fold(0, vertical);
            let view_down = height_matrix[(row_idx + 1)..].iter().fold(0, vertical);

            let score = view_left * view_right * view_up * view_down;
            max_scenic_score = score.max(max_scenic_score);
        }
    }

    println!("best scenic score: {max_scenic_score}");
}

fn matrixify(input: &str) -> Vec<Vec<u8>> {
    let mut rows: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        let columns = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        rows.push(columns);
    }
    rows
}
