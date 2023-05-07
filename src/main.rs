use triangulation::cli;



fn main() {
    let result = cli::execute();
    match result {
        Err(e) => eprintln!("{:?}", e),
        _ => {}
    }
}
