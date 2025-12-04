use std::fs::read_to_string;

fn main() {
    let mut zeroes: i16 = 0;
    let input = match read_to_string("input.txt") {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err),
    };

    let mut dial_num: i16 = 50;
    for (dir, num) in input.lines().map(|l| l.split_at(1)) {
        let mut n = num.parse::<u16>().unwrap() as i16;
        if n > 99 {
            zeroes += n / 100;
            n %= 100
        }

        if n == 0 {
            continue;
        }

        match dir {
            "L" => dial_num -= n,
            "R" => dial_num += n,
            _ => panic!("Unknown direction {:?}", dir),
        }

        if dial_num < 0 {
            if dial_num + n != 0 {
                zeroes += 1;
            }
            dial_num += 100;
        } else if dial_num > 99 {
            zeroes += 1;
            dial_num -= 100;
        } else if dial_num == 0 {
            zeroes += 1;
        }
    }
    println!("{:?}", zeroes)
}
