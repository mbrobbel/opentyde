use crate::parser::river;
use petgraph::{
    dot::{Config, Dot},
    graph::Graph,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn river_dot(input: String) -> String {
    match river::river_type(&input) {
        Ok((_, river)) => {
            let graph: Graph<String, &str> = river.into();
            format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]))
        }
        Err(x) => format!("{:#?}", x),
    }
}
