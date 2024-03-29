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

    // Reset Scheduler
    scheduler.init();

    for instruction in instruction_vector {
        let result = match instruction[0].as_str() {
            "in" => scheduler.init(),
            "cr" => scheduler.create(instruction[1].parse().expect("Invalid Argument")),
            "de" => scheduler.destroy(instruction[1].parse().expect("Invalid Argument")),
            "rq" => scheduler.request(
                instruction[1].parse().expect("Invalid Argument"),
                instruction[2].parse().expect("Invalid Argument"),
            ),
            "rl" => scheduler.release(
                instruction[1].parse().expect("Invalid Argument"),
                instruction[2].parse().expect("Invalid Argument"),
            ),
            "to" => scheduler.timeout(),
            _ => None,
        };

        output.push(result);
    }

    output
}

fn write_output(filename: &str, output: &[Vec<Option<usize>>]) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let last_index = output.len() - 1;

    for (i, batch) in output.iter().enumerate() {
        let batch_str = batch
            .iter()
            .map(|i| {
                i.map(|i| i.to_string())
                    .unwrap_or_else(|| String::from("-1"))
            })
            .collect::<Vec<_>>()
            .join(" ");

        if i == last_index {
            write!(writer, "{batch_str} ")?;
        } else {
            write!(writer, "{batch_str}\r\n")?;
        }
    }

    Ok(())
}

pub fn interactive_shell(input_filename: &str, output_filename: &str) -> Result<(), &'static str> {
    let mut scheduler = Scheduler::new();

    let instruction_vectors = match read_file(input_filename) {
        Ok(instruction_vectors) => instruction_vectors,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                return Err("Input file (input.txt) not found in project root")
            }
            _ => {
                eprintln!("{}", error);
                return Err("Error reading input file");
            }
        },
    };

    let output: Vec<Vec<Option<usize>>> = instruction_vectors
        .iter()
        .map(|instruction_vector| handle_instruction_vector(&mut scheduler, instruction_vector))
        .collect();

    match write_output(output_filename, &output) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("{}", error);
            return Err("Error writing to output file");
        }
    }

    Ok(())
}
