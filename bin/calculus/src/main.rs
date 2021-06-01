extern crate calculus_parser;

#[cfg(not(test))]
fn main() {
    println!(
        "{:?}",
        calculus_parser::calculator::ExprParser::new()
            .parse("22")
            .unwrap()
    );
    println!("nmsl");

    println!(
        "{:?}",
        calculus_parser::calculator::ExprParser::new()
            .parse("22.22312")
            .unwrap()
    );

    println!(
        "{:?}",
        calculus_parser::calculator::ExprParser::new()
            .parse("7.321E-3")
            .unwrap()
    );

    println!(
        "{:?}",
        calculus_parser::calculator::ExprParser::new()
            .parse("4e-11")
            .unwrap()
    );

    println!(
        "{:?}",
        calculus_parser::calculator::ExprParser::new()
            .parse("4e-11 * (3 - 2)")
            .unwrap()
    );
}
