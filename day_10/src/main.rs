use pathfinding::prelude::astar;

#[derive(Debug)]
struct Machine {
    goal: State,
    buttons: Vec<Vec<u32>>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State(Vec<i32>);

impl State {
    fn next_nodes(&self, buttons: Vec<Vec<u32>>) -> Vec<(State, u32)> {
        let mut result: Vec<(State, u32)> = Vec::new();
        for button in buttons {
            let mut new_state = self.0.clone();
            for idx in button {
                new_state[idx as usize] -= 1
            }
            if new_state.iter().any(|x| *x < 0) {
                continue;
            }
            result.push((State(new_state), 1));
        }
        result
    }

    fn cost(&self) -> u32 {
        *self.0.iter().max().unwrap() as u32
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let machines: Vec<Machine> = input
        .lines()
        .map(|l| {
            let mut split = l.split(' ');
            let buttons: Vec<Vec<u32>> = split
                .clone()
                .skip(1)
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
            let goal = State(
                split
                    .next_back()
                    .unwrap()
                    .replace(['{', '}'], " ")
                    .trim()
                    .split(',')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
            );

            Machine { goal, buttons }
        })
        .collect();

    let mut total = 0;
    for machine in machines {
        println!("{:?}", machine);
        let path = astar(
            &machine.goal,
            |n| n.next_nodes(machine.buttons.clone()),
            |n| n.cost(),
            |n| *n == State(vec![0; machine.goal.0.len()]),
        );
        total += path.unwrap().1;
    }
    println!("{:?}", total);
}
