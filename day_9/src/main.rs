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

    let potential_areas = cords.iter().combinations(2).sorted_by(|b, a| {
        (((a[1].0 - a[0].0).abs() + 1) * ((a[1].1 - a[0].1).abs() + 1))
            .cmp(&(((b[1].0 - b[0].0).abs() + 1) * ((b[1].1 - b[0].1).abs() + 1)))
    });

    println!("{:?}", cords);
    println!("{:?}", potential_areas);
    for pair in potential_areas {
        if cords.iter().any(|cord| {
            ((cord.0 > pair[0].0 && cord.0 < pair[1].0)
                || (cord.0 > pair[1].0 && cord.0 < pair[0].0))
                && ((cord.1 > pair[0].1 && cord.1 < pair[1].1)
                    || (cord.1 > pair[1].1 && cord.1 < pair[0].1))
        }) {
            continue;
        }
        println!(
            "{:?}",
            (((pair[1].0 - pair[0].0).abs() + 1) * ((pair[1].1 - pair[0].1).abs() + 1))
        );
        return;
    }
}
