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
        .fold(BTreeMap::new(), |mut map, (k, v): (u64, u64)| {
            map.entry(k).and_modify(|e| *e = (*e).max(v)).or_insert(v);
            map
        });
    let fresh: Vec<u64> = input
        .lines()
        .skip_while(|l| !l.is_empty()) // Jump past the ranges
        .skip(1) // Jump the empty line
        .map(|l| l.parse().unwrap())
        .filter(|id| fresh_id_ranges.range(0..=*id).any(|(_, end)| end >= id))
        .collect();
    println!("{:?}", fresh.len());
}
