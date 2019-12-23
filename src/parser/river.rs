use crate::{
    parser::{nonempty_comma_list, r#type, space_opt, usize},
    river::{Complexity, RiverParameters},
    River,
};
use nom::{
    branch::alt,
    character::complete::char,
    combinator::map,
    sequence::{separated_pair, tuple},
    IResult,
};

/// Returns a River type parser.
#[allow(clippy::needless_lifetimes)] // rust-lang/rust-clippy/issues/2944
fn river_type_parser<'a, F>(name: &'a str, inner: F) -> impl Fn(&'a str) -> IResult<&'a str, River>
where
    F: Fn((River, RiverParameters)) -> River,
{
    map(
        r#type(
            name,
            separated_pair(river_type, space_opt(char(',')), river_parameters),
        ),
        inner,
    )
}

/// Parses a RiverParameters.
fn river_parameters(input: &str) -> IResult<&str, RiverParameters> {
    map(
        tuple((
            usize,
            space_opt(char(',')),
            usize,
            space_opt(char(',')),
            usize,
        )),
        |(elements, _, _complexity, _, userbits): (usize, _, usize, _, usize)| {
            RiverParameters {
                elements,
                complexity: Complexity::default(), // TODO
                userbits,
            }
        },
    )(input)
}

/// Parses a Bits<b>.
fn bits(input: &str) -> IResult<&str, River> {
    map(r#type("Bits", usize), River::Bits)(input)
}

/// Parses a Root<T, N, C, U>.
fn root(input: &str) -> IResult<&str, River> {
    river_type_parser("Root", |(river_type, river_parameters)| {
        River::Root(Box::new(river_type), river_parameters)
    })(input)
}

/// Parses a Group<T, U, ...>.
fn group(input: &str) -> IResult<&str, River> {
    map(
        r#type("Group", nonempty_comma_list(river_type)),
        River::Group,
    )(input)
}

/// Parses a Dim<T, N, C, U>.
fn dim(input: &str) -> IResult<&str, River> {
    river_type_parser("Dim", |(river_type, river_parameters)| {
        River::Dim(Box::new(river_type), river_parameters)
    })(input)
}

/// Parses a New<T, N, C, U>.
fn new(input: &str) -> IResult<&str, River> {
    river_type_parser("New", |(river_type, river_parameters)| {
        River::New(Box::new(river_type), river_parameters)
    })(input)
}

/// Parses a Flat<T, N, C, U>.
fn flat(input: &str) -> IResult<&str, River> {
    river_type_parser("Flat", |(river_type, river_parameters)| {
        River::Flat(Box::new(river_type), river_parameters)
    })(input)
}

/// Parses a Rev<T, N, C, U>.
fn rev(input: &str) -> IResult<&str, River> {
    river_type_parser("Rev", |(river_type, river_parameters)| {
        River::Rev(Box::new(river_type), river_parameters)
    })(input)
}

/// Parses a Union<T, U, ...>.
fn r#union(input: &str) -> IResult<&str, River> {
    map(
        r#type("Union", nonempty_comma_list(river_type)),
        River::Union,
    )(input)
}

/// Parses a River type.
pub fn river_type(input: &str) -> IResult<&str, River> {
    alt((r#union, rev, flat, new, dim, group, root, bits))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::river::{Complexity, RiverParameters};

    #[test]
    fn parse_river_parameters() {
        assert_eq!(
            river_parameters("3, 4, 5"),
            Ok((
                "",
                RiverParameters {
                    elements: 3,
                    complexity: Complexity::default(), // TODO
                    userbits: 5
                }
            ))
        );
    }

    #[test]
    fn parse_bits() {
        assert_eq!(bits("Bits<8>"), Ok(("", River::Bits(8))));
        assert!(bits("Bits<>").is_err());
        assert!(bits("bits<8>").is_err());
    }

    #[test]
    fn parse_root() {
        assert_eq!(
            root("Root<Bits<8>, 1, 2, 3>"),
            Ok((
                "",
                River::Root(
                    Box::new(River::Bits(8)),
                    RiverParameters {
                        elements: 1,
                        complexity: Complexity::default(), // TODO
                        userbits: 3
                    }
                )
            ))
        );
    }

    #[test]
    fn parse_group() {
        assert_eq!(
            group("Group<Bits<4>, Bits<8>>"),
            Ok(("", River::Group(vec![River::Bits(4), River::Bits(8)])))
        );
    }

    #[test]
    fn parse_dim() {
        assert_eq!(
            dim("Dim<Bits<8>, 1, 2, 3>"),
            Ok((
                "",
                River::Dim(
                    Box::new(River::Bits(8)),
                    RiverParameters {
                        elements: 1,
                        complexity: Complexity::default(), // TODO
                        userbits: 3
                    }
                )
            ))
        );
    }

    #[test]
    fn parse_new() {
        assert_eq!(
            new("New<Bits<7>, 3, 2, 1>"),
            Ok((
                "",
                River::New(
                    Box::new(River::Bits(7)),
                    RiverParameters {
                        elements: 3,
                        complexity: Complexity::default(), // TODO
                        userbits: 1
                    }
                )
            ))
        );
    }

    #[test]
    fn parse_flat() {
        assert_eq!(
            flat("Flat<New<Bits<7>, 3, 2, 1>, 1, 2, 3>"),
            Ok((
                "",
                River::Flat(
                    Box::new(River::New(
                        Box::new(River::Bits(7)),
                        RiverParameters {
                            elements: 3,
                            complexity: Complexity::default(), // TODO
                            userbits: 1
                        }
                    )),
                    RiverParameters {
                        elements: 1,
                        complexity: Complexity::default(), // TODO
                        userbits: 3
                    }
                )
            ))
        );
    }

    #[test]
    fn parse_rev() {
        assert_eq!(
            rev("Rev<Bits<8>, 11, 22, 33>"),
            Ok((
                "",
                River::Rev(
                    Box::new(River::Bits(8)),
                    RiverParameters {
                        elements: 11,
                        complexity: Complexity::default(), // TODO
                        userbits: 33
                    }
                )
            ))
        );
    }

    #[test]
    fn parse_union() {
        assert_eq!(
            union("Union<Bits<8>, Bits<4>>"),
            Ok(("", River::Union(vec![River::Bits(8), River::Bits(4)])))
        );
    }

    #[test]
    fn parse_river_type() {
        assert_eq!(river_type("Bits<8>"), Ok(("", River::Bits(8))));
    }
}
