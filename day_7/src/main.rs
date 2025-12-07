#![feature(ascii_char)]

const START: u8 = b'S';
const EMPTY: u8 = b'.';
const BEAM: u8 = b'|';
const SPLIT: u8 = b'^';

#[derive(Clone, Copy)]
enum GridItem {
    Char(u8),
    BeamCount(u64),
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines: Vec<Vec<GridItem>> = input
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|&char| match char {
                    START | BEAM => GridItem::BeamCount(1),
                    _ => GridItem::Char(char),
                })
                .collect()
        })
        .collect();
    let mut last_line = lines[0].clone();
    for line in lines.iter_mut().skip(1) {
        for idx in 0..line.len() {
            let above_beam_count: u64 = match last_line[idx] {
                GridItem::BeamCount(count) => count,
                _ => 0,
            };

            match (line[idx], above_beam_count > 0) {
                (GridItem::Char(EMPTY), true) => line[idx] = GridItem::BeamCount(above_beam_count),
                (GridItem::Char(SPLIT), true) => {
                    line[idx - 1] = match line[idx - 1] {
                        GridItem::BeamCount(orig) => GridItem::BeamCount(above_beam_count + orig),
                        _ => GridItem::BeamCount(above_beam_count),
                    };
                    line[idx + 1] = match line[idx + 1] {
                        GridItem::BeamCount(orig) => GridItem::BeamCount(above_beam_count + orig),
                        _ => GridItem::BeamCount(above_beam_count),
                    };
                }
                (GridItem::BeamCount(orig), true) => {
                    line[idx] = GridItem::BeamCount(above_beam_count + orig)
                }
                _ => (),
            }
        }
        last_line = line.to_vec();
    }
    println!(
        "{:?}",
        last_line
            .iter()
            .map(|&item| match item {
                GridItem::BeamCount(count) => count,
                _ => 0,
            })
            .sum::<u64>()
    )
}
