const DIGIT_SIZE: usize = 12;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "{:?}",
        input
            .lines()
            .map(|line| {
                let mut sum: u64 = 0;
                let mut digits_to_find = DIGIT_SIZE;
                let mut last_highest_char_index = 0; // bad name, this is index after
                while digits_to_find > 0 {
                    let search_end = line.len() - digits_to_find + 1;
                    let next = line
                        .char_indices()
                        .skip(last_highest_char_index) // skip to our last highest number
                        .take_while(|(idx, _)| *idx < search_end) // go search for the next one that leaves us enough remaining digits
                        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0))) // take the highest next one, but closest to the front
                        .unwrap();
                    sum = (sum * 10) + next.1.to_digit(10).unwrap() as u64;
                    last_highest_char_index = next.0 + 1;
                    digits_to_find -= 1;
                }
                sum
            })
            .sum::<u64>()
    );
}
