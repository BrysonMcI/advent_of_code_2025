use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

type Graph<'a> = HashMap<[i64; 3], Vec<[i64; 3]>>;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Connection {
    from: [i64; 3],
    to: [i64; 3],
    distance: i64,
    depth: usize,
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // Connect 1000 pairs of closest junction boxes
    // Figure out top 3 largest circuits and multiply them
    // Input is 3d cords, xyz

    let input = std::fs::read_to_string("input.txt").unwrap();
    let cords_list: Vec<[i64; 3]> = input
        .lines()
        .map(|l| {
            let mut split = l.splitn(3, ',');
            [
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            ]
        })
        .collect();
    // kd tree to cheap calc closest
    let kdtree = kd_tree::KdTree::build(cords_list.clone());
    // calc lowest on all and load min heap
    let mut connections: Graph =
        Graph::from_iter(cords_list.iter().cloned().map(|cord| (cord, vec![cord])));
    let mut min_heap: BinaryHeap<Connection> = cords_list
        .iter()
        .map(|cord| {
            let to = kdtree.nearests(cord, 2)[1];
            Connection {
                from: *cord,
                to: *to.item,
                distance: to.squared_distance,
                depth: 1,
            }
        })
        .collect();

    // pop, connect and recalc next lower on that coord back into minheap
    loop {
        let a = min_heap.pop().unwrap();
        let b = min_heap.pop().unwrap();
        if let Some(val) = connections.get_mut(&a.from) {
            val.push(a.to);
        };
        if let Some(val) = connections.get_mut(&a.to) {
            val.push(a.from);
        };
        let to = kdtree.nearests(&a.from, a.depth + 2)[a.depth + 1];
        let new_a = Connection {
            from: a.from,
            to: *to.item,
            distance: to.squared_distance,
            depth: a.depth + 1,
        };
        min_heap.push(new_a);
        let to = kdtree.nearests(&b.from, b.depth + 2)[b.depth + 1];
        let new_b = Connection {
            from: b.from,
            to: *to.item,
            distance: to.squared_distance,
            depth: b.depth + 1,
        };
        min_heap.push(new_b);

        let groups = connections.values().cloned().collect::<Vec<_>>();
        // This would be way faster if I tracked the components over time and which component
        // each node is in and merged on connection until all components had been merged to one
        let circuits =
            pathfinding::undirected::connected_components::separate_components(groups.as_slice());
        if circuits.1.iter().all(|a| *a == circuits.1[0]) {
            println!("{:?}", a.from[0] * a.to[0]);
            return;
        }
    }
}
