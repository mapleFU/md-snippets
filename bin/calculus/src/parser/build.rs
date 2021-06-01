extern crate lalrpop;

fn main() {
    // If debug, using unit_test
    lalrpop::Configuration::new()
        .emit_comments(true)
        .force_build(true)
        .log_info()
        .process_current_dir()
        .unwrap();
}
