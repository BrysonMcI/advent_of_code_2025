use pathfinding::prelude::count_paths;
use std::collections::HashMap;

const START_NODE: &str = "svr";
const END_NODE: &str = "out";
const FFT_NODE: &str = "fft";
const DAC_NODE: &str = "dac";

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let empty: Vec<String> = Vec::new();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (key, vals) = line.split_once(":").unwrap();
        let node = key.to_string();
        let connections: Vec<String> = vals.split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(node, connections);
    }

    let start_to_fft_no_dac = count_paths(
        START_NODE.to_string(),
        |n: &String| {
            graph
                .get(n)
                .unwrap_or(&empty)
                .iter()
                .filter(|a| *a != DAC_NODE)
                .cloned()
        },
        |n: &String| *n == FFT_NODE,
    );
    let fft_to_dac = count_paths(
        FFT_NODE.to_string(),
        |n: &String| graph.get(n).unwrap_or(&empty).iter().cloned(),
        |n: &String| *n == DAC_NODE,
    );
    let dac_to_end_no_fft = count_paths(
        DAC_NODE.to_string(),
        |n: &String| {
            graph
                .get(n)
                .unwrap_or(&empty)
                .iter()
                .filter(|a| *a != FFT_NODE)
                .cloned()
        },
        |n: &String| *n == END_NODE,
    );

    let start_to_dac_no_fft = count_paths(
        START_NODE.to_string(),
        |n: &String| {
            graph
                .get(n)
                .unwrap_or(&empty)
                .iter()
                .filter(|a| *a != FFT_NODE)
                .cloned()
        },
        |n: &String| *n == DAC_NODE,
    );
    let dac_to_fft = count_paths(
        DAC_NODE.to_string(),
        |n: &String| graph.get(n).unwrap_or(&empty).iter().cloned(),
        |n: &String| *n == FFT_NODE,
    );
    let fft_to_end_no_dac = count_paths(
        FFT_NODE.to_string(),
        |n: &String| {
            graph
                .get(n)
                .unwrap_or(&empty)
                .iter()
                .filter(|a| *a != DAC_NODE)
                .cloned()
        },
        |n: &String| *n == END_NODE,
    );

    println!(
        "{:?}",
        (start_to_fft_no_dac * fft_to_dac * dac_to_end_no_fft)
            + (start_to_dac_no_fft * dac_to_fft * fft_to_end_no_dac)
    );
}
