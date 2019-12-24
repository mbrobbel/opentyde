use crate::parser::river;
use crate::river::River;
use petgraph::{graph::Graph, visit::EdgeRef, Direction};
use std::{
    fmt,
    fmt::{Display, Formatter},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn river_dot(input: String) -> String {
    match river::river_type(&input) {
        Ok((_, river)) => {
            let graph: Graph<River, &str> = river.into();
            let dot = Into::<Dot>::into(graph);
            format!("{}", dot)
        }
        Err(x) => format!("{:#?}", x),
    }
}

struct Dot<'a> {
    graph: Graph<River, &'a str>,
}

impl<'a> From<Graph<River, &'a str>> for Dot<'a> {
    fn from(graph: Graph<River, &'a str>) -> Dot<'a> {
        Dot { graph }
    }
}

impl<'a> Display for Dot<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "digraph {{")?;

        self.graph
            .externals(Direction::Incoming)
            .try_for_each(|root| self.graph[root].fmt_dot(f, root, &self.graph))?;

        self.graph.edge_references().try_for_each(|edge| {
            writeln!(f, "\"{:?}\" -> \"{:?}\"", edge.source(), edge.target())
        })?;

        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot() {
        let river = river::river_type("Root<Group<Bits<3>,Dim<Bits<4>,1,2,3>>, 1, 2, 3>")
            .map(|(_, river)| river)
            .unwrap();
        let graph: Graph<River, &str> = river.into();
        let dot = Into::<Dot>::into(graph);
        eprintln!("{}", dot);
    }
}
