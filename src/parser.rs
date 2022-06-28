use nom::{bytes::complete::tag, IResult};

fn parse(input: &str) -> IResult<&str, &str>
{
    tag("int")(input)
}

#[cfg(test)]
mod tests
{
    use crate::parser::parse;

    #[test]
    fn int()
    {
        assert_eq!(parse("int 1"), Ok((" 1", "int")));
    }
}
