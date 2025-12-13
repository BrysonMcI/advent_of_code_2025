use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
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

    let blocker_cords: Vec<(i64, i64)> = cords.iter().circular_tuple_windows().flat_map(|(first, next)| match first.0 == next.0 {
        true => match first.1 < next.1 {
            true => (first.1..=next.1).map(|y| (first.0, y)).collect::<Vec<(i64, i64)>>(),
            false => (next.1..=first.1).map(|y| (first.0, y)).collect::<Vec<(i64, i64)>>(),
        }
        false => match first.0 < next.0 {
            true => (first.0..=next.0).map(|x| (x, first.1)).collect::<Vec<(i64, i64)>>(),
            false => (next.0..=first.0).map(|x| (x, first.1)).collect::<Vec<(i64, i64)>>(),
        }
    }).collect();

    
    for pair in potential_areas {
        if blocker_cords.iter().any(|cord| {
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
