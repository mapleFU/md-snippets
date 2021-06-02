#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate lazy_static;

lalrpop_mod!(pub calculator); // synthesized by LALRPOP

pub mod calculator_ast;

#[test]
fn expr_calculator() {
    assert!(calculator::NumParser::new().parse("22").is_ok());
    assert!(calculator::FactorParser::new().parse("22").is_ok());
    assert!(calculator::ExprParser::new().parse("22").is_ok());

    assert_eq!(
        22,
        calculator::ExprParser::new()
            .parse("22")
            .unwrap()
            .eval()
            .as_i64()
    );
    assert_eq!(
        22 + 22 * 22,
        calculator::ExprParser::new()
            .parse("22 + 22 * 22")
            .unwrap()
            .eval()
            .as_i64()
    );
    assert_eq!(
        22 + 22 * 22 - 22 / 2,
        calculator::ExprParser::new()
            .parse("22 + 22 * 22 - 22 / 2")
            .unwrap()
            .eval()
            .as_i64()
    );

    assert_eq!(
        22 + 22 * (22 - 22) / 2,
        calculator::ExprParser::new()
            .parse("22 + 22 * (22 - 22) / 2")
            .unwrap()
            .eval()
            .as_i64()
    );
    assert_eq!(
        22,
        calculator::ExprParser::new()
            .parse("22 + 22 * ((((((22 - 22)))))) / 2")
            .unwrap()
            .eval()
            .as_i64()
    );

    assert_eq!(
        -22,
        calculator::ExprParser::new()
            .parse("-22 + 22 * ((((((22 - 22)))))) / 2")
            .unwrap()
            .eval()
            .as_i64()
    );

    assert_eq!(
        -22,
        calculator::ExprParser::new()
            .parse("a = -22")
            .unwrap()
            .eval()
            .as_i64()
    );

    assert_eq!(
        1 - 22 * 33,
        calculator::ExprParser::new()
            .parse("a = 1 -22 * 33")
            .unwrap()
            .eval()
            .as_i64()
    );

    assert_eq!(
        true,
        calculator::ExprParser::new()
            .parse("-100 -200 -85 * 5 == 1 -22 * 33")
            .unwrap()
            .eval()
            .as_bool()
    );
}

#[test]
fn stmt_test() {
    assert_eq!(
        3,
        calculator::ListParser::new()
            .parse("a = 2 ; if ( a == 2 )  then { 3 }")
            .unwrap()
            .eval()
            .as_i64()
    );

    assert_eq!(
        3,
        calculator::ListParser::new()
            .parse("a = 2 ; if ( a == 2 )  then { 3 } else { 5 }")
            .unwrap()
            .eval()
            .as_i64()
    );

    assert_eq!(
        3,
        calculator::ListParser::new()
            .parse("a = 1 ; if ( a == 2 )  then { 3 } else { 5 }")
            .unwrap()
            .eval()
            .as_i64()
    );
}