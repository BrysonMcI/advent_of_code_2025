use std::collections::BTreeMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let fresh_id_ranges: BTreeMap<u64, u64> = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (start, end) = l.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .fold(BTreeMap::new(), |mut map, (start, end): (u64, u64)| {
            let mut merged_start = start;
            let mut merged_end = end;

            let overlapping: Vec<_> = map
                .range(..=end)
                .filter(|(s, e)| !(**e < start || **s > end))
                .map(|(s, e)| (*s, *e))
                .collect();
            for (s, e) in &overlapping {
                merged_start = merged_start.min(*s);
                merged_end = merged_end.max(*e);
            }

            for (s, _) in overlapping {
                map.remove(&s);
            }
            map.insert(merged_start, merged_end);
            map
        });
    let fresh_id_count: u64 = fresh_id_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum();
    println!("{:?}", fresh_id_count);
}
