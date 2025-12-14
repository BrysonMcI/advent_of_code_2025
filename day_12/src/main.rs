#[derive(Debug)]
struct Shape {
    index: usize,
    cells: [[bool; 3]; 3],
    filled_cells: usize,
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    quantities: Vec<usize>,
}

#[derive(Debug)]
struct Problem {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

fn parse_problem(input: &str) -> Problem {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();

    let mut lines = input.lines().peekable();

    // -------- shapes --------
    while let Some(line) = lines.peek() {
        let line = line.trim();

        // stop when line no longer looks like a shape header
        if !line.ends_with(':') || !line[..line.len() - 1].chars().all(|c| c.is_ascii_digit()) {
            break;
        }

        let header = lines.next().unwrap();
        let index = header[..header.len() - 1].parse().unwrap_or(0);

        let mut cells = [[false; 3]; 3];
        let mut filled_cells = 0;

        for y in 0..3 {
            if let Some(row) = lines.next() {
                for (x, c) in row.chars().take(3).enumerate() {
                    if c == '#' {
                        cells[y][x] = true;
                        filled_cells += 1;
                    }
                }
            }
        }

        shapes.push(Shape {
            index,
            cells,
            filled_cells,
        });

        // consume optional blank line
        if let Some(l) = lines.peek()
            && l.trim().is_empty()
        {
            lines.next();
        }
    }

    // -------- regions --------
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (dims, rest) = match line.split_once(':') {
            Some(v) => v,
            None => continue,
        };

        let (w, h) = match dims.split_once('x') {
            Some(v) => v,
            None => continue,
        };

        let width = w.trim().parse().unwrap_or(0);
        let height = h.trim().parse().unwrap_or(0);

        let quantities = rest
            .split_whitespace()
            .filter_map(|q| q.parse::<usize>().ok())
            .collect();

        regions.push(Region {
            width,
            height,
            quantities,
        });
    }

    Problem { shapes, regions }
}

fn main() {
    let problem = parse_problem(&std::fs::read_to_string("input.txt").unwrap());
    let total = problem
        .regions
        .iter()
        .filter(|region| {
            region.quantities.iter().sum::<usize>() <= (region.height / 3) * (region.width / 3)
        })
        .filter(|region| {
            region.height * region.width
                > region
                    .quantities
                    .iter()
                    .enumerate()
                    .map(|(idx, num)| problem.shapes[idx].filled_cells * *num)
                    .sum()
        })
        .count();
    println!("{:?}", total);
}

// Check overall space of the region can fit tight fit squares for the number of requested presents
