#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub integers);

#[test]
fn i32() {
    assert!(integers::TermParser::new().parse("22").is_ok());
    assert!(integers::TermParser::new().parse("{\"_\": !\"_i32\" \"22\"}").is_ok());
    assert!(integers::TermParser::new().parse("\"_\": !\"_i32\" \"22\"}").is_err());
    assert!(integers::TermParser::new().parse("{_\": !\"_i32\" \"22\"}").is_err());
    assert!(integers::TermParser::new().parse("{\"\": !\"_i32\" \"22\"}").is_err());
    assert!(integers::TermParser::new().parse("{\"_: !\"_i32\" \"22\"}").is_err());
    assert!(integers::TermParser::new().parse("{\"_\" !\"_i32\" \"22\"}").is_err());
    assert!(integers::TermParser::new().parse("{\"_\": \"_i32\" \"22\"}").is_err());
    assert!(integers::TermParser::new().parse("{\"_\": !_i32\" \"22\"}").is_err());
    assert!(integers::TermParser::new().parse("{\"_\": !_i32\" \"22\"}").is_err());
    assert!(integers::TermParser::new().parse("{\"_\": !\"i32\" \"22\"}").is_err());
    assert!(integers::TermParser::new().parse("{\"_\": !\"_i32 \"22\"}").is_err());
    assert!(integers::TermParser::new().parse("{\"_\": !\"_i32\" 22\"}").is_err());
    assert!(integers::TermParser::new().parse("{\"_\": !\"_i32\" \"22}").is_err());
    assert!(integers::TermParser::new().parse("{\"_\": !\"_i32\" \"22\"").is_err());
}
