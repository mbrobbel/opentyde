use crate::parser::river;
use crate::river::River;
use std::{
    fmt,
    fmt::{Display, Formatter},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn river_dot(input: String) -> String {
    match river::river_type(&input) {
        Ok((_, river)) => {
            let dot: DotGraph = river.as_ref().into();
            format!("{}", dot)
        }
        Err(x) => format!("{:#?}", x),
    }
}

struct DotGraph<'a>(&'a River);

impl<'a> From<&'a River> for DotGraph<'a> {
    fn from(river: &'a River) -> DotGraph<'a> {
        DotGraph(river)
    }
}

impl<'a> Display for DotGraph<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "digraph {{")?;
        self.0.write_nodes(f)?;
        self.0.write_edges(f)?;
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot() {
        let river = river::river_type("Root<Bits<3>, 1, 2, 3>")
            .map(|(_, river)| river)
            .unwrap();
        let _: DotGraph = river.as_ref().into();
    }
}
