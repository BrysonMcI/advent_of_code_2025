fn calc(operator: u8, problem: &[u64]) -> u64 {
    match operator {
        b'+' => problem.iter().sum::<u64>(),
        b'*' => problem
            .iter()
            .copied()
            .reduce(|accu, el| accu * el)
            .unwrap(),
        bad => panic!("bad operator: {:?}", bad),
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut byte_lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut total: u64 = 0;

    let mut cur_operator: u8 = b' ';
    let mut cur_problem: Vec<u64> = Vec::with_capacity(5); // based on max input size
    // right to left doesn't seem to matter? Could .rev() this but I get the same answer
    for (idx, el) in (*byte_lines.pop().unwrap()).iter().enumerate() {
        if byte_lines.iter().all(|line| line[idx] == b' ') {
            total += calc(cur_operator, &cur_problem);
            cur_problem.clear();
            cur_operator = b' ';
            continue;
        }
        if *el != b' ' {
            cur_operator = *el;
        }
        cur_problem.push(
            byte_lines
                .iter()
                .map(|line| line[idx])
                .filter(|b| *b != b' ')
                .fold(0_u64, |acc, b| acc * 10 + (b - b'0') as u64),
        );
    }
    total += calc(cur_operator, &cur_problem);

    println!("{:?}", total)
}
