
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calculator); // synthesized by LALRPOP


#[test]
fn calculator() {
    assert!(calculator::NumParser::new().parse("22").is_ok());
    assert!(calculator::FactorParser::new().parse("22").is_ok());
    assert!(calculator::ExprParser::new().parse("22").is_ok());
}

#[cfg(not(test))]
fn main() {
    let var = calculator::ExprParser::new().parse("22").unwrap();
    println!("{}", var);

    let var = calculator::ExprParser::new().parse("22 + 22").unwrap();
    println!("{}", var);


    let var = calculator::ExprParser::new().parse("22 + 22 * 22").unwrap();
    println!("{}", var);

    let var = calculator::ExprParser::new().parse("22 + 22 * 22 - 22 / 2").unwrap();
    println!("{}", var);
}