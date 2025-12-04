use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = match read_to_string("input.txt") {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err),
    };

    let mut factors_store: HashMap<usize, Vec<usize>> = HashMap::with_capacity(12);
    for i in 1..12 {
        // Part 1
        // if i % 2 != 0 {
        //     continue;
        // }
        // factors_store.insert(i, vec![i / 2]);
        factors_store.insert(i, (1..i).filter(|j| i % j == 0).collect::<Vec<usize>>());
    }

    let invalid_id_sum = input
        .split(",") // split to ranges
        .map(|r_str| r_str.split_once('-').unwrap()) // split range
        .map(|r_str_iter: (&str, &str)| {
            r_str_iter.0.parse::<u64>().unwrap()..=r_str_iter.1.parse::<u64>().unwrap()
        }) // convert to range
        .flat_map(|r| {
            r.map(|num| {
                let num_str = num.to_string();
                let num_str_len = num_str.len();
                let bytes = num_str.as_bytes();

                // Part 1
                // if num_str_len % 2 != 0 {
                //     return 0;
                // }

                for f in factors_store.get(&num_str_len).unwrap() {
                    let first = &bytes[0..*f];
                    if bytes.chunks(*f).all(|chunk| chunk == first) {
                        return num;
                    }
                }
                0
            }) // map this number to itself if it is invalid
        }) // convert range to sum of invalid ids
        .sum::<u64>();

    println!("{:?}", invalid_id_sum);
}
