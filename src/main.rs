use scheduler::process::interactive_shell;

fn main() -> std::io::Result<()> {
    interactive_shell("input.txt", "output.txt")
}
