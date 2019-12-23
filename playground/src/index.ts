import { EditorState } from "@codemirror/next/state";
import { keymap } from "@codemirror/next/keymap";
import { baseKeymap, selectAll } from "@codemirror/next/commands";
import { ViewUpdate, EditorView, ViewPlugin } from "@codemirror/next/view";
import { history, redo, undo } from "@codemirror/next/history";
import { Text } from "@codemirror/next/text";

import * as d3 from "d3";
import "d3-graphviz";

import "firacode";
import "./index.css";

import { river, river_dot } from "../../Cargo.toml";

let output = new EditorView({
  state: EditorState.create({
    doc: river("Bits<1>")
  })
});

let river_parser = ViewPlugin.create(view => ({
  update(update: ViewUpdate) {
    if (update.docChanged) {
      const doc = update.state.doc.toString();
      const parsed = river(doc);
      output.dispatch(
        output.state.t().replace(0, output.state.doc.length, parsed)
      );

      if (!parsed.startsWith("Error")) {
        graph.renderDot(river_dot(doc));
      }
    }
  }
}));

let graph = d3
  .select("#graph")
  .graphviz()
  .renderDot(river_dot("Bits<1>"));

let input = new EditorView({
  state: EditorState.create({
    doc: "Bits<1>",
    extensions: [
      history(),
      keymap({ "Mod-z": undo, "Mod-Shift-z": redo, "Mod-a": selectAll }),
      keymap(baseKeymap),
      river_parser.extension
    ]
  })
});
document.getElementById("input").appendChild(input.dom);
document.getElementById("output").appendChild(output.dom);
