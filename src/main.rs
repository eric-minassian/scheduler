use crate::scheduler::Scheduler;
use std::fs::File;
use std::io::{BufWriter, Write};

pub mod scheduler;

fn main() -> std::io::Result<()> {
    let mut scheduler = Scheduler::new();
    let input = std::fs::read_to_string("input.txt")?;

    let mut output: Vec<Vec<Option<usize>>> = Vec::new();
    let mut current_batch: Vec<Option<usize>> = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            if !current_batch.is_empty() {
                output.push(current_batch);
                current_batch = Vec::new();
            }
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let result = match parts[0] {
            "in" => scheduler.init(),
            "cr" => scheduler.create(parts[1].parse().unwrap()),
            "de" => scheduler.destroy(parts[1].parse().unwrap()),
            "rq" => scheduler.request(parts[1].parse().unwrap(), parts[2].parse().unwrap()),
            "rl" => scheduler.release(parts[1].parse().unwrap(), parts[2].parse().unwrap()),
            "to" => scheduler.timeout(),
            _ => None,
        };

        current_batch.push(result);
    }

    if !current_batch.is_empty() {
        output.push(current_batch);
    }

    let output_file = File::create("output.txt")?;
    let mut writer = BufWriter::new(output_file);

    for batch in output {
        let batch_str = batch
            .iter()
            .map(|i| i.map_or_else(|| "-1".to_string(), |i| i.to_string()))
            .collect::<Vec<_>>()
            .join(" ");

        writeln!(writer, "{}", batch_str)?;
    }

    Ok(())
}
