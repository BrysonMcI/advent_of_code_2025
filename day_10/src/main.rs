use pathfinding::prelude::dijkstra;

struct Machine {
    goal: State,
    buttons: Vec<Vec<u32>>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State(Vec<bool>);

impl State {
    fn next_nodes(&self, buttons: Vec<Vec<u32>>) -> Vec<(State, u32)> {
        let mut result: Vec<(State, u32)> = Vec::new();
        for button in buttons {
            let mut new_state = self.0.clone();
            for idx in button {
                new_state[idx as usize] = !new_state[idx as usize]
            }
            result.push((State(new_state), 1));
        }
        result
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let machines: Vec<Machine> = input
        .lines()
        .map(|l| {
            let mut split = l.split(' ');
            let goal = State(
                split
                    .next()
                    .unwrap()
                    .as_bytes()
                    .iter()
                    .filter_map(|b| match b {
                        b'.' => Some(false),
                        b'#' => Some(true),
                        _ => None,
                    })
                    .collect(),
            );
            let buttons: Vec<Vec<u32>> = split
                .take_while(|s| s.chars().nth(0).unwrap() == '(')
                .map(|s| s.chars())
                .map(|s| {
                    s.filter_map(|c| match c.is_ascii_digit() {
                        true => Some(c.to_digit(10).unwrap()),
                        false => None,
                    })
                    .collect()
                })
                .collect();
            Machine { goal, buttons }
        })
        .collect();

    let mut total = 0;
    for machine in machines {
        let path = dijkstra(
            &machine.goal,
            |n| n.next_nodes(machine.buttons.clone()),
            |n| *n == State(vec![false; machine.goal.0.len()]),
        );
        total += path.unwrap().1;
    }
    println!("{:?}", total);
}
