pub mod ast {

    /// High level data types.
    #[derive(Debug, PartialEq)]
    pub enum Data {
        Prim(usize),             // Prim<B>
        Struct(Vec<Data>),       // Struct<T, U, ...>
        Tuple(Box<Data>, usize), // Tuple<T, n>
        Seq(Box<Data>),          // Seq<T>
        Variant(Vec<Data>),      // Variant<T, U, ...>
    }
}

pub mod parser {
    use crate::ast::Data;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, digit1, space0},
        combinator::{map, map_res},
        multi::separated_nonempty_list,
        sequence::{delimited, preceded, separated_pair, terminated},
        IResult,
    };

    /// Returns a parser function to parse a Type<_>.
    fn r#type<'a, T, F>(name: &'a str, inner: F) -> impl Fn(&'a str) -> IResult<&'a str, T>
    where
        F: Fn(&'a str) -> IResult<&'a str, T>,
    {
        preceded(tag(name), delimited(char('<'), inner, char('>')))
    }

    /// Returns a parser function which allow space characters after provided
    /// parser.
    fn space_opt<'a, T, F>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, T>
    where
        F: Fn(&'a str) -> IResult<&'a str, T>,
    {
        terminated(inner, space0)
    }

    /// Returns a parser function to parse non-empty comma-separated space
    /// optional lists.
    fn nonempty_comma_list<'a, T, F>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, Vec<T>>
    where
        F: Fn(&'a str) -> IResult<&'a str, T>,
    {
        separated_nonempty_list(space_opt(char(',')), inner)
    }

    /// Parses some digits to a usize.
    fn usize(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    /// Parses a Prim<B>.
    fn prim(input: &str) -> IResult<&str, Data> {
        map(r#type("Prim", usize), Data::Prim)(input)
    }

    /// Parses a Struct<T, U, ...>.
    fn r#struct(input: &str) -> IResult<&str, Data> {
        map(
            r#type("Struct", nonempty_comma_list(data_type)),
            Data::Struct,
        )(input)
    }

    /// Parses a Tuple<T, n>.
    fn tuple(input: &str) -> IResult<&str, Data> {
        map(
            r#type(
                "Tuple",
                separated_pair(data_type, space_opt(char(',')), usize),
            ),
            |(data_type, count)| Data::Tuple(Box::new(data_type), count),
        )(input)
    }

    /// Parses a Seq<T>.
    fn seq(input: &str) -> IResult<&str, Data> {
        map(r#type("Seq", data_type), |data_type| {
            Data::Seq(Box::new(data_type))
        })(input)
    }

    /// Parses a Variant<T, U, ...>.
    fn variant(input: &str) -> IResult<&str, Data> {
        map(
            r#type("Variant", nonempty_comma_list(data_type)),
            Data::Variant,
        )(input)
    }

    /// Parses a Data type.
    pub fn data_type(input: &str) -> IResult<&str, Data> {
        alt((variant, seq, tuple, r#struct, prim))(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::ast::Data;

        #[test]
        fn parse_prim() {
            assert_eq!(prim("Prim<8>"), Ok(("", Data::Prim(8))));
            assert!(prim("Prim<>").is_err());
            assert!(prim("prim<8>").is_err());
        }

        #[test]
        fn parse_struct() {
            assert_eq!(
                r#struct("Struct<Prim<3>>"),
                Ok(("", Data::Struct(vec![Data::Prim(3)])))
            );
            assert_eq!(
                r#struct("Struct<Struct<Prim<3>>>"),
                Ok(("", Data::Struct(vec![Data::Struct(vec![Data::Prim(3)])])))
            );
            assert_eq!(
                r#struct("Struct<Struct<Prim<3>,Prim<8>>>"),
                Ok((
                    "",
                    Data::Struct(vec![Data::Struct(vec![Data::Prim(3), Data::Prim(8)])])
                ))
            );
            assert_eq!(
                r#struct("Struct<Struct<Prim<3>,Prim<8>>>"),
                r#struct("Struct<Struct<Prim<3>, Prim<8>>>"),
            );
        }

        #[test]
        fn parse_tuple() {
            assert_eq!(
                tuple("Tuple<Prim<8>,4>"),
                Ok(("", Data::Tuple(Box::new(Data::Prim(8)), 4)))
            );
        }

        #[test]
        fn parse_seq() {
            assert_eq!(
                seq("Seq<Tuple<Prim<8>,4>>"),
                Ok((
                    "",
                    Data::Seq(Box::new(Data::Tuple(Box::new(Data::Prim(8)), 4)))
                ))
            );
        }

        #[test]
        fn parse_variant() {
            assert_eq!(
                variant("Variant<Prim<8>, Seq<Tuple<Prim<8>,4>>>"),
                Ok((
                    "",
                    Data::Variant(vec![
                        Data::Prim(8),
                        Data::Seq(Box::new(Data::Tuple(Box::new(Data::Prim(8)), 4)))
                    ])
                ))
            );
        }

        #[test]
        fn data_type_parser() {
            assert_eq!(data_type("Prim<8>"), Ok(("", Data::Prim(8))));
            assert_eq!(
                data_type("Struct<Prim<4>, Prim<4>>"),
                Ok(("", Data::Struct(vec![Data::Prim(4), Data::Prim(4)])))
            );
            assert_eq!(
                data_type("Tuple<Prim<8>, 4>"),
                Ok(("", Data::Tuple(Box::new(Data::Prim(8)), 4)))
            );
            assert_eq!(
                data_type("Seq<Prim<4>>"),
                Ok(("", Data::Seq(Box::new(Data::Prim(4)))))
            );
            assert_eq!(
                data_type("Variant<Prim<4>,Prim<4>>"),
                Ok(("", Data::Variant(vec![Data::Prim(4), Data::Prim(4)])))
            );
        }
    }
}
