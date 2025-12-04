const DIGIT_SIZE: usize = 12;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "{:?}",
        input
            .lines()
            .map(|line| {
                let mut sum: u64 = 0;
                let mut last_highest_char_index = 0;
                for remaining in (1..=DIGIT_SIZE).rev() {
                    let search_end = line.len() - remaining + 1;
                    let next = line[last_highest_char_index..search_end] // skip to our last highest number and ignore enough numbers to fill the rest of the digits
                        .char_indices()
                        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0))) // take the highest next one, but closest to the front
                        .unwrap();
                    sum = (sum * 10) + next.1.to_digit(10).unwrap() as u64;
                    last_highest_char_index += next.0 + 1;
                }
                sum
            })
            .sum::<u64>()
    );
}
