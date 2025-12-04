const DIGIT_SIZE: usize = 12;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "{:?}",
        input
            .lines()
            .map(|line| {
                let mut highest_chars: [u64; DIGIT_SIZE] = [0; DIGIT_SIZE];
                let mut digits_to_find = DIGIT_SIZE;
                let mut last_highest_char_index = 0; // bad name, this is index after
                while digits_to_find > 0 {
                    let next = line
                        .char_indices()
                        .skip(last_highest_char_index)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .skip(digits_to_find - 1)
                        .max_by(|a, b| a.1.cmp(&b.1))
                        .unwrap();
                    highest_chars[DIGIT_SIZE - digits_to_find] =
                        next.1.to_digit(10).unwrap() as u64;
                    last_highest_char_index = next.0 + 1;
                    digits_to_find -= 1;
                }

                let mut sum: u64 = 0;
                for (i, digit) in highest_chars.iter().enumerate() {
                    sum += digit * (10_u64.pow((DIGIT_SIZE - 1 - i) as u32));
                }
                sum
            })
            .sum::<u64>()
    );
}
