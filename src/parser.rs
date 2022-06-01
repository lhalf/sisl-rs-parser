use nom::{bytes::complete::tag, IResult};

fn foo(input: &str) -> IResult<&str, &str>
{
    tag("int")(input)
}

#[cfg(test)]
mod tests
{
    use crate::parser::foo;

    #[test]
    fn int()
    {
        assert_eq!(foo("int bar"), Ok((" bar", "int")));
    }
}
