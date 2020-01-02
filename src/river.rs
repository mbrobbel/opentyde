use std::{
    convert::AsRef,
    fmt,
    fmt::{Display, Formatter},
};

/// Parameters of River types.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RiverParameters {
    /// N: number of elements per handshake.
    pub elements: Option<usize>,
    /// C: complexity level.
    pub complexity: Option<usize>,
    /// U: number of user bits.
    pub userbits: Option<usize>,
}

impl Default for RiverParameters {
    fn default() -> Self {
        RiverParameters {
            elements: None,
            complexity: None,
            userbits: None,
        }
    }
}

impl Display for RiverParameters {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.elements {
            Some(elements) => write!(f, "N={},", elements),
            None => write!(f, "N,"),
        }?;
        match self.complexity {
            Some(complexity) => write!(f, "C={},", complexity),
            None => write!(f, "C,"),
        }?;
        match self.userbits {
            Some(userbits) => write!(f, "U={}", userbits),
            None => write!(f, "U"),
        }
    }
}

/// River types.
#[derive(Clone, Debug, PartialEq)]
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
    /// Rev<T, N, C, U>
    Rev(Box<River>, RiverParameters),
    /// Union<T, U, ...>
    Union(Vec<River>),
}

impl Display for River {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let info = match self {
            River::Bits(bits) => format!("Bits<{}>", bits),
            River::Root(_, params) => format!("Root<{}>", params),
            River::Group(_) => "Group".to_string(),
            River::Dim(_, params) => format!("Dim<{}>", params),
            River::New(_, params) => format!("New<{}>", params),
            River::Rev(_, params) => format!("Rev<{}>", params),
            River::Union(_) => "Union".to_string(),
        };
        write!(f, "{}", info)
    }
}

impl AsRef<River> for River {
    fn as_ref(&self) -> &River {
        self
    }
}

// TODO(mb): add a trait for these methods
impl River {
    fn id(&self) -> String {
        format!("{:p}", self)
    }
    fn as_node(&self) -> String {
        format!(
            "\"{}\" [style=filled, fillcolor=\"{}\", label=\"{}\"]",
            self.id(),
            self.color(),
            self.to_string()
        )
    }
    fn edge(&self, target: &River) -> String {
        format!("\"{}\" -> \"{}\"", self.id(), target.id())
    }
    fn color(&self) -> String {
        format!(
            "#{}",
            match self {
                River::Bits(_) => "EE926B",
                River::Root(_, _) => "7DBFA7",
                River::Dim(_, _) => "DA90C0",
                River::Group(_) => "90A0C7",
                River::New(_, _) => "A6D854",
                River::Union(_) => "E7C595",
                River::Rev(_, _) => "B3B3B3",
            }
        )
    }
}

impl River {
    pub fn write_nodes(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{}", self.as_node())?;
        match self {
            River::Bits(_) => Ok(()),
            River::New(river, _) => {
                writeln!(f, "}} subgraph cluster_{} {{", self.id())?;
                river.write_nodes(f)
            }
            River::Root(river, _) | River::Dim(river, _) | River::Rev(river, _) => {
                writeln!(f, "subgraph cluster_{} {{", self.id())?;
                match **river {
                    River::New(_, _) | River::Dim(_, _) => writeln!(f, "style=\"dotted\""),
                    _ => writeln!(f, "style=\"solid\""),
                }?;
                river.write_nodes(f)?;
                writeln!(f, "}}")
            }
            River::Group(rivers) | River::Union(rivers) => {
                rivers
                    .iter()
                    .filter(|river| match river {
                        River::New(_, _) => false,
                        _ => true,
                    })
                    .try_for_each(|river| river.write_nodes(f))?;
                rivers
                    .iter()
                    .filter(|river| match river {
                        River::New(_, _) => true,
                        _ => false,
                    })
                    .try_for_each(|river| river.write_nodes(f))
            }
        }
    }

    pub fn write_edges(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            River::Bits(_) => Ok(()),
            River::New(river, _)
            | River::Root(river, _)
            | River::Dim(river, _)
            | River::Rev(river, _) => {
                writeln!(f, "{}", self.edge(river))?;
                river.write_edges(f)
            }
            River::Group(rivers) | River::Union(rivers) => rivers.iter().try_for_each(|river| {
                writeln!(f, "{}", self.edge(river))?;
                river.write_edges(f)
            }),
        }
    }
}
