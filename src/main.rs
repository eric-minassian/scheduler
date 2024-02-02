use std::process::ExitCode;

use scheduler::process::interactive_shell;

fn main() -> ExitCode {
    match interactive_shell("input.txt", "output.txt") {
        Ok(_) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{}", message);
            ExitCode::FAILURE
        }
    }
}
