use std::fs::read_to_string;

fn main() {
    let mut zeroes: u16 = 0;
    let input = match read_to_string("input.txt") {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err),
    };

    let mut dial_num: i16 = 50;
    for (dir, num) in input.lines().map(|l| l.split_at(1)) {
        match dir {
            "L" => dial_num -= num.parse::<u16>().unwrap() as i16,
            "R" => dial_num += num.parse::<u16>().unwrap() as i16,
            _ => panic!("Unknown direction {:?}", dir),
        }

        if dial_num < 0 {
            dial_num += 100;
        }

        dial_num %= 100;
        if dial_num == 0 {
            zeroes += 1
        }
    }
    println!("{:?}", zeroes)
}
