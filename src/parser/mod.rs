use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, space0},
    combinator::map_res,
    multi::separated_nonempty_list,
    sequence::{delimited, preceded, terminated},
    IResult,
};

pub mod data;
pub mod river;

/// Returns a parser function to parse a Type<_>.
pub(crate) fn r#type<'a, T, F>(name: &'a str, inner: F) -> impl Fn(&'a str) -> IResult<&'a str, T>
where
    F: Fn(&'a str) -> IResult<&'a str, T>,
{
    preceded(tag(name), delimited(char('<'), inner, char('>')))
}

/// Returns a parser function which allow space characters after provided
/// parser.
pub(crate) fn space_opt<'a, T, F>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, T>
where
    F: Fn(&'a str) -> IResult<&'a str, T>,
{
    terminated(inner, space0)
}

/// Returns a parser function to parse non-empty comma-separated space
/// optional lists.
pub(crate) fn nonempty_comma_list<'a, T, F>(
    inner: F,
) -> impl Fn(&'a str) -> IResult<&'a str, Vec<T>>
where
    F: Fn(&'a str) -> IResult<&'a str, T>,
{
    separated_nonempty_list(space_opt(char(',')), inner)
}

/// Parses some digits to a usize.
pub(crate) fn usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}
