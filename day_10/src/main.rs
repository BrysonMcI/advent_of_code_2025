use std::collections::HashMap;

#[derive(Debug, Clone)] // Add Clone
struct Machine {
    goal: Vec<u32>,
    buttons: Vec<Vec<usize>>,
}

impl Machine {
    fn with_goal(&self, new_goal: Vec<u32>) -> Machine {
        Machine {
            goal: new_goal,
            buttons: self.buttons.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State(Vec<u32>);

impl State {
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

fn solve_recursive(machine: &Machine, memo: &mut HashMap<Vec<u32>, Option<u32>>) -> Option<u32> {
    if machine.goal.iter().all(|&g| g == 0) {
        return Some(0);
    }

    if let Some(&result) = memo.get(&machine.goal) {
        return result;
    }

    let n_buttons = machine.buttons.len();
    let mut min_cost = None;

    for mask in 0..(1 << n_buttons) {
        let mut pressed = vec![0; n_buttons];
        let mut num_pressed = 0;

        for (i, press) in pressed.iter_mut().enumerate() {
            if (mask >> i) & 1 == 1 {
                *press = 1;
                num_pressed += 1;
            }
        }

        let state = State(pressed);
        let current = state.current_val(machine);

        if current
            .iter()
            .zip(&machine.goal)
            .all(|(c, g)| c % 2 == g % 2 && c <= g)
        {
            let new_goal: Vec<u32> = machine
                .goal
                .iter()
                .zip(&current)
                .map(|(g, c)| (g - c) / 2)
                .collect();

            if let Some(rec_cost) = solve_recursive(&machine.with_goal(new_goal), memo) {
                let total = 2 * rec_cost + num_pressed;
                min_cost = Some(min_cost.map_or(total, |mc: u32| mc.min(total)));
            }
        }
    }

    memo.insert(machine.goal.clone(), min_cost);
    min_cost
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
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
        let mut memo = HashMap::new();
        let cost = solve_recursive(&machine, &mut memo).unwrap();
        total += cost;
    }
    println!("{:?}", total);
}
