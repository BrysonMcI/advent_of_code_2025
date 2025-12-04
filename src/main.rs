fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "{:?}",
        input
            .lines()
            .map(|line| {
                let highest_char = line
                    .char_indices()
                    .rev()
                    .skip(1)
                    .max_by(|a, b| a.1.cmp(&b.1))
                    .unwrap();
                let next_highest = line.chars().skip(highest_char.0 + 1).max().unwrap();
                highest_char.1.to_digit(10).unwrap() * 10 + next_highest.to_digit(10).unwrap()
            })
            .sum::<u32>()
    );
}
