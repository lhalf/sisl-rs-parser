#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub integers);

#[test]
fn integers() {
    assert!(integers::TermParser::new().parse("22").is_ok());
    assert!(integers::TermParser::new().parse("{22}").is_ok());
    assert!(integers::TermParser::new().parse("{{{{22}}}}").is_ok());
    assert!(integers::TermParser::new().parse("{{22}").is_err());
}
