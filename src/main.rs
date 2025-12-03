use std::fs::read_to_string;

fn main() {
    let input = match read_to_string("input.txt") {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err),
    };

    let invalid_id_sum = input
        .split(",") // split to ranges
        .map(|r_str| r_str.split('-').collect()) // split range
        .map(|r_str_iter: Vec<&str>| {
            r_str_iter[0].parse::<u128>().unwrap()..=r_str_iter[1].parse::<u128>().unwrap()
        }) // convert to range
        .filter_map(|r| {
            r.map(|num| {
                let num_str = num.to_string();
                let num_str_len = num_str.len();
                let mut factors = Vec::new();
                for i in 1..num_str_len {
                    if num_str_len % i == 0 {
                        factors.push(i);
                    }
                }
                for f in factors {
                    if num_str
                        .chars()
                        .collect::<Vec<char>>() // turn string into vec of chars
                        .chunks(f) // because vecs can be chunked
                        .collect::<Vec<&[char]>>() // then turn chunks into an vec
                        .windows(2) // because vecs can be windowed
                        .all(|p| p[0] == p[1])
                    {
                        return num;
                    }
                }
                0
            }) // map this number to itself if it is invalid
            .reduce(|a, b| a + b) // sum this ranges invalid ids
        }) // convert range to sum of invalid ids
        .reduce(|a, b| a + b) // sum invalid ids
        .unwrap();

    println!("{:?}", invalid_id_sum);
}
