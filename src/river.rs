/// Complexity levels.
#[derive(Debug, PartialEq)]
pub enum Complexity {
    One,
}

impl Default for Complexity {
    fn default() -> Self {
        Complexity::One
    }
}

/// Parameters of River types.
#[derive(Debug, PartialEq)]
pub struct RiverParameters {
    /// N: number of elements per handshake.
    pub elements: usize,
    /// C: complexity level.
    pub complexity: Complexity,
    /// U: number of user bits.
    pub userbits: usize,
}

impl Default for RiverParameters {
    fn default() -> Self {
        RiverParameters {
            elements: 1,
            complexity: Complexity::default(),
            userbits: 0,
        }
    }
}

/// River types.
#[derive(Debug, PartialEq)]
pub enum River {
    /// Bits<b>
    Bits(usize),
    /// Root<T, N, C, U>
    Root(Box<River>, RiverParameters),
    /// Group<T, U, ...>
    Group(Vec<River>),
    /// Dim<T, N, C, U>
    Dim(Box<River>, RiverParameters),
    /// New<T, N, C, U>
    New(Box<River>, RiverParameters),
    /// Flat<T, N, C, U>
    Flat(Box<River>, RiverParameters),
    /// Rev<T, N, C, U>
    Rev(Box<River>, RiverParameters),
    /// Union<T, U, ...>
    Union(Vec<River>),
}
