use nom::{bytes::complete::tag, IResult};

fn parse(input: &str) -> IResult<&str, &str>
{
    tag("!i8 ")(input)
}

#[cfg(test)]
mod tests
{
    use crate::parser::parse;

    #[test]
    fn i8()
    {
        assert_eq!(parse("!i8 10"), Ok(("10", "!i8 ")));
    }
}
