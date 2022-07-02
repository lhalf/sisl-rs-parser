use nom::{bytes::complete::tag, IResult};

#[allow(dead_code)]
fn parse(input: &str) -> IResult<&str, i8>
{
    let (input, _) = tag("!i8 ")(input)?;
    let (input, output_i8) = parse_i8(input)?;
    Ok((input, output_i8))
}

#[allow(dead_code)]
fn parse_i8(input: &str) -> IResult<&str, i8>
{
    Ok(("", input.parse::<i8>().unwrap()))
}

#[cfg(test)]
mod tests
{
    use crate::parser::parse;

    #[test]
    fn i8()
    {
        assert_eq!(parse("!i8 10"), Ok(("", 10 as i8)));
    }
}
