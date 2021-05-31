#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calculator); // synthesized by LALRPOP

pub mod calculator_ast;

#[test]
fn calculator() {
    assert!(calculator::NumParser::new().parse("22").is_ok());
    assert!(calculator::FactorParser::new().parse("22").is_ok());
    assert!(calculator::ExprParser::new().parse("22").is_ok());

    assert_eq!(
        22,
        calculator::ExprParser::new().parse("22").unwrap().as_i64()
    );
    assert_eq!(
        22 + 22 * 22,
        calculator::ExprParser::new()
            .parse("22 + 22 * 22")
            .unwrap()
            .as_i64()
    );
    assert_eq!(
        22 + 22 * 22 - 22 / 2,
        calculator::ExprParser::new()
            .parse("22 + 22 * 22 - 22 / 2")
            .unwrap()
            .as_i64()
    );

    assert_eq!(
        22 + 22 * (22 - 22) / 2,
        calculator::ExprParser::new()
            .parse("22 + 22 * (22 - 22) / 2")
            .unwrap()
            .as_i64()
    );
    assert_eq!(
        22,
        calculator::ExprParser::new()
            .parse("22 + 22 * ((((((22 - 22)))))) / 2")
            .unwrap()
            .as_i64()
    );

    assert_eq!(
        -22,
        calculator::ExprParser::new()
            .parse("-22 + 22 * ((((((22 - 22)))))) / 2")
            .unwrap()
            .as_i64()
    );
}
