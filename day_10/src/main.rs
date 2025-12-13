use pathfinding::prelude::astar;

#[derive(Debug)]
struct Machine {
    goal: Vec<u32>,
    buttons: Vec<Vec<usize>>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State(Vec<u32>);

impl State {
    fn next_nodes(&self, machine: &Machine) -> Vec<(State, u32)> {
        println!("{:?}", self);
        let mut result: Vec<(State, u32)> = Vec::with_capacity(machine.buttons.len());
        for idx in 0..self.0.len() {
            let mut new_presses = self.0.clone();
            new_presses[idx] += 1;
            let new_state = State(new_presses);
            if new_state
                .current_val(machine)
                .iter()
                .zip(machine.goal.clone())
                .any(|(cur, goal)| *cur > goal)
            {
                continue;
            }
            result.push((new_state, 1));
        }
        result
    }

    fn min_cost_to_end(&self, machine: &Machine) -> u32 {
        self.current_val(machine)
            .iter()
            .zip(machine.goal.clone())
            .map(|(cur, goal)| goal - cur)
            .max()
            .unwrap()
    }

    fn current_val(&self, machine: &Machine) -> Vec<u32> {
        let mut val = vec![0; machine.goal.len()];
        for (but_idx, presses) in self.0.iter().enumerate() {
            for val_idx in machine.buttons.get(but_idx).unwrap() {
                *val.get_mut(*val_idx).unwrap() += presses;
            }
        }
        val
    }
}

fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let machines: Vec<Machine> = input
        .lines()
        .map(|l| {
            let mut split = l.split(' ');
            let buttons: Vec<Vec<usize>> = split
                .clone()
                .skip(1)
                .take_while(|s| s.starts_with('('))
                .map(|s| s.chars())
                .map(|s| {
                    s.filter_map(|c| match c.is_ascii_digit() {
                        true => Some(c.to_digit(10).unwrap() as usize),
                        false => None,
                    })
                    .collect()
                })
                .collect();
            let goal = split
                .next_back()
                .unwrap()
                .replace(['{', '}'], " ")
                .trim()
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            Machine { goal, buttons }
        })
        .collect();

    let mut total = 0;
    for machine in machines {
        println!("{:?}", machine);
        let start = State(vec![0; machine.buttons.len()]);
        let path = astar(
            &start,
            |n| n.next_nodes(&machine),
            |n| n.min_cost_to_end(&machine),
            |n| n.current_val(&machine) == machine.goal,
        );
        total += path.unwrap().1;
    }
    println!("{:?}", total);
}
