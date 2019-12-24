use petgraph::{graph::NodeIndex, Graph};
use std::{
    fmt,
    fmt::{Display, Formatter},
};

/// Parameters of River types.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RiverParameters {
    /// N: number of elements per handshake.
    pub elements: usize,
    /// C: complexity level.
    pub complexity: usize,
    /// U: number of user bits.
    pub userbits: usize,
}

impl Default for RiverParameters {
    fn default() -> Self {
        RiverParameters {
            elements: 1,
            complexity: 0,
            userbits: 0,
        }
    }
}

impl Display for RiverParameters {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "N={},C={},U={}",
            self.elements, self.complexity, self.userbits
        )
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
    /// Flat<T, N, C, U>
    Flat(Box<River>, RiverParameters),
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
            River::Flat(_, params) => format!("Flat<{}>", params),
            River::Rev(_, params) => format!("Rev<{}>", params),
            River::Union(_) => "Union".to_string(),
        };
        write!(f, "{}", info)
    }
}

impl River {
    /// Add this River to the given Graph. Returns the modified Graph. An edge
    /// is added when a parent node is provided.
    fn add_to_graph<'a>(
        &self,
        mut graph: Graph<River, &'a str>,
        parent: Option<NodeIndex>,
    ) -> Graph<River, &'a str> {
        let node = graph.add_node(self.to_owned());
        if let Some(parent) = parent {
            graph.add_edge(parent, node, "");
        }
        match self {
            River::Bits(_) => graph,
            River::Root(river_type, _)
            | River::Dim(river_type, _)
            | River::New(river_type, _)
            | River::Flat(river_type, _)
            | River::Rev(river_type, _) => river_type.add_to_graph(graph, Some(node)),
            River::Group(rivers) | River::Union(rivers) => rivers
                .iter()
                .fold(graph, |graph, river| river.add_to_graph(graph, Some(node))),
        }
    }

    fn get_color(&self) -> String {
        String::from(match self {
            River::Bits(_) => "#EE926B",
            River::Root(_, _) => "#7DBFA7",
            River::Dim(_, _) => "#DA90C0",
            River::Group(_) => "#90A0C7",
            _ => todo!(),
        })
    }

    pub fn fmt_dot(
        &self,
        f: &mut Formatter,
        node_id: NodeIndex,
        graph: &Graph<River, &str>,
    ) -> fmt::Result {
        match self {
            River::Bits(_) => writeln!(
                f,
                "\"{:?}\" [shape=ellipse, width=1, style=filled, fillcolor=\"{}\", label=\"{}\"]",
                // "\"{:?}\" [width=2, style=\"rounded,filled\", fillcolor=\"#EE926B\", label=<\
                //  <TABLE BORDER=\"0\"> \
                //  <TR bgcolor='white'><TD>Bits</TD></TR>\
                //  <TR><TD>{}</TD></TR>\
                //  </TABLE>>]",
                //\"Bits\\n{}\"]",
                node_id,
                self.get_color(),
                self.to_string()
            ),
            River::Root(_, _params) | River::Dim(_, _params) => {
                writeln!(f, "\"{:?}\" [shape=ellipse, width=1, style=filled, fillcolor=\"{}\", label=\"{}\"]", node_id, self.get_color(), self.to_string())?;
                writeln!(f, "subgraph cluster_{} {{", node_id.index())?;
                graph
                    .neighbors(node_id)
                    .try_for_each(|node| graph[node].fmt_dot(f, node, &graph))?;
                writeln!(f, "}}")
            }
            River::Group(_) => {
                writeln!(f, "\"{:?}\" [shape=ellipse, width=1, style=filled, fillcolor=\"{}\", label=\"{}\"]", node_id, self.get_color(), self.to_string())?;
                graph
                    .neighbors(node_id)
                    .try_for_each(|node| graph[node].fmt_dot(f, node, &graph))
            }
            _ => write!(f, "\"{}\"", node_id.index()),
        }
    }
}

impl<'a> From<River> for Graph<River, &'a str> {
    fn from(river: River) -> Graph<River, &'a str> {
        river.add_to_graph(Graph::<River, &str>::new(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use petgraph::dot::{Config, Dot};

    #[test]
    fn river_graph() {
        let river = River::Root(Box::new(River::Bits(8)), RiverParameters::default());
        let _graph: Graph<River, &str> = river.into();

        let river = River::Group(vec![River::Bits(8), River::Bits(4), River::Bits(2)]);
        let graph = Graph::<River, &str>::new();
        let _ = river.add_to_graph(graph, None);
    }
}
