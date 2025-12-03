use std::fs::read_to_string;

fn main() {
    let input = match read_to_string("input.txt") {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err),
    };

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

                for i in 1..num_str_len {
                    if num_str_len % i != 0 {
                        continue;
                    }
                    let first = &bytes[0..i];
                    if bytes.chunks(i).all(|chunk| chunk == first) {
                        return num;
                    }
                }
                0
            }) // map this number to itself if it is invalid
        }) // convert range to sum of invalid ids
        .sum::<u64>();

    println!("{:?}", invalid_id_sum);
}
