pub fn print_error(msg: impl Into<String>) {
    eprintln!("[tesql error]: {}", msg.into());
}
