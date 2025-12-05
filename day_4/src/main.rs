const ADJACENT_TO_BLOCK: u8 = 4;
const PAPER_ROLL: u8 = b'@';
const EMPTY: u8 = b'.';

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut total = 0;
    loop {
        // This would be much faster if it used memoization to store the adjacent count, then removal updates those and new removals are a single scan
        // but, it was late.
        let mut can_move: Vec<(usize, usize)> = Vec::new();
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, element) in row.iter().enumerate() {
                if *element == EMPTY {
                    continue;
                }
                let rows_to_check = grid
                    .iter()
                    .skip(row_idx.saturating_sub(1))
                    .take(if row_idx == 0 { 2 } else { 3 });
                let adjacent: u8 = rows_to_check
                    .map(|row| {
                        let elements = row
                            .iter()
                            .skip(col_idx.saturating_sub(1))
                            .take(if col_idx == 0 { 2 } else { 3 });
                        elements
                            .map(|el| match *el == PAPER_ROLL {
                                true => 1,
                                false => 0,
                            })
                            .sum::<u8>()
                    })
                    .sum();
                if adjacent - 1 < ADJACENT_TO_BLOCK {
                    can_move.push((row_idx, col_idx));
                }
            }
        }
        if can_move.is_empty() {
            break;
        }
        total += can_move.len();
        // PART 1 answer, break here
        for (row, col) in can_move {
            grid[row][col] = EMPTY;
        }
    }
    println!("{:?}", total);
}
