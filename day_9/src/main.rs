use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let cords: Vec<(i64, i64)> = input
        .lines()
        .map(|l| {
            let split = l.split_once(',').unwrap();
            (split.0.parse().unwrap(), split.1.parse().unwrap())
        })
        .collect();

    let biggest_area = cords
        .iter()
        .combinations(2)
        .map(|pair| ((pair[1].0 - pair[0].0).abs() + 1) * ((pair[1].1 - pair[0].1).abs() + 1))
        .max()
        .unwrap();

    println!("{:?}", biggest_area);
}
