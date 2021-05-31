extern crate calculus_parser;

#[cfg(not(test))]
fn main() {
    println!(
        "{}",
        calculus_parser::calculator::ExprParser::new()
            .parse("22")
            .unwrap()
    );
    println!("nmsl");
}
