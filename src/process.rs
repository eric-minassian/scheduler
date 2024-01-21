use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;

use crate::scheduler::Scheduler;

fn read_file(filename: &str) -> io::Result<Vec<Vec<Vec<String>>>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut instruction_vectors = Vec::new();
    let mut current_vector = Vec::new();

    for line in reader.lines() {
        let line: String = line?;
        if line.trim().is_empty() {
            if !current_vector.is_empty() {
                instruction_vectors.push(current_vector);
                current_vector = Vec::new();
            }
        } else {
            let words = line.split_whitespace().map(String::from).collect();
            current_vector.push(words);
        }
    }

    if !current_vector.is_empty() {
        instruction_vectors.push(current_vector);
    }

    Ok(instruction_vectors)
}

fn handle_instruction_vector(
    scheduler: &mut Scheduler,
    instruction_vector: &Vec<Vec<String>>,
) -> Vec<Option<usize>> {
    let mut output = Vec::new();

    for instruction in instruction_vector {
        let result = match instruction[0].as_str() {
            "in" => scheduler.init(),
            "cr" => scheduler.create(instruction[1].parse().unwrap()),
            "de" => scheduler.destroy(instruction[1].parse().unwrap()),
            "rq" => scheduler.request(
                instruction[1].parse().unwrap(),
                instruction[2].parse().unwrap(),
            ),
            "rl" => scheduler.release(
                instruction[1].parse().unwrap(),
                instruction[2].parse().unwrap(),
            ),
            "to" => scheduler.timeout(),
            _ => None,
        };

        output.push(result);
    }

    output
}

fn write_output(filename: &str, output: Vec<Vec<Option<usize>>>) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let last_index = output.len() - 1;

    for (i, batch) in output.iter().enumerate() {
        let batch_str = batch
            .iter()
            .map(|i| i.map(|i| i.to_string()).unwrap_or(String::from("-1")))
            .collect::<Vec<_>>()
            .join(" ");

        if i == last_index {
            write!(writer, "{} ", batch_str)?;
        } else {
            write!(writer, "{}\r\n", batch_str)?;
        }
    }

    Ok(())
}

pub fn interactive_shell(input_filename: &str, output_filename: &str) -> io::Result<()> {
    let mut scheduler = Scheduler::new();

    let instruction_vectors = read_file(input_filename)?;

    let output = instruction_vectors
        .iter()
        .map(|instruction_vector| handle_instruction_vector(&mut scheduler, instruction_vector))
        .collect();

    write_output(output_filename, output)?;

    Ok(())
}
