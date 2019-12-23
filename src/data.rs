/// High level data types.
#[derive(Debug, PartialEq)]
pub enum Data {
    /// Empty
    Empty,
    /// Prim<B>
    Prim(usize),
    /// Struct<T, U, ...>
    Struct(Vec<Data>),
    /// Tuple<T, n>
    Tuple(Box<Data>, usize),
    /// Seq<T>
    Seq(Box<Data>),
    /// Variant<T, U, ...>
    Variant(Vec<Data>),
}
