const PROBLEM_SIZE: usize = 5;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut split_lines: Vec<_> = input.lines().map(|l| l.split_whitespace()).collect();
    let mut problems: Vec<Vec<&str>> = Vec::new();
    loop {
        let mut problem: Vec<&str> = Vec::with_capacity(PROBLEM_SIZE);
        for l in split_lines.iter_mut() {
            match l.next() {
                Some(item) => problem.push(item),
                None => break,
            }
        }
        if problem.is_empty() {
            break;
        }

        problems.push(problem);
    }
    println!("{:?}", problems);
    let mut total: u64 = 0;
    for problem in problems {
        total += match problem[PROBLEM_SIZE - 1] {
            "*" => problem[0..PROBLEM_SIZE - 1]
                .iter()
                .map(|s| s.parse::<u64>().unwrap())
                .reduce(|accu, el| accu * el)
                .unwrap(),
            "+" => problem[0..PROBLEM_SIZE - 1]
                .iter()
                .map(|s| s.parse::<u64>().unwrap())
                .sum(),
            bad => panic!("bad operator: {:?}", bad),
        };
    }
    println!("{:?}", total)
}
