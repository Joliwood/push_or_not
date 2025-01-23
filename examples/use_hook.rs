use push_or_not::execute_push_or_not;

fn main() {
    if let Err(e) = execute_push_or_not(false) {
        eprintln!("Hook execution failed: {}", e);
        std::process::exit(1);
    }
}
