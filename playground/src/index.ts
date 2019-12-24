import { EditorState } from "@codemirror/next/state";
import { keymap } from "@codemirror/next/keymap";
import { baseKeymap, selectAll } from "@codemirror/next/commands";
import { ViewUpdate, EditorView, ViewPlugin } from "@codemirror/next/view";
import { history, redo, undo } from "@codemirror/next/history";
import { Text } from "@codemirror/next/text";

import * as d3 from "d3-selection";
import "d3-graphviz";

import "firacode";
import "./index.css";

import { river, river_dot } from "../../Cargo.toml";

// const start = "Group<Bits<4>,Dim<Group<Rev<Bits<3>,1,1,1>,Bits<8>>,1,1,1>>";
const start = "Root<Group<Bits<3>,Dim<Bits<4>,1,2,3>>, 1, 2, 3>";

const editor = (fn, extensions) =>
  new EditorView({
    state: EditorState.create({
      doc: fn(start),
      extensions: extensions
    })
  });

const append = (name, editor) => {
  document.getElementById(name).appendChild(editor.dom);
}

const river_parser = ViewPlugin.create(view => ({
  update(update: ViewUpdate) {
    if (update.docChanged) {
      const doc = update.state.doc.toString();
      const parsed = river(doc);
      output.dispatch(
        output.state.t().replace(0, output.state.doc.length, parsed)
      );

      if (!parsed.startsWith("Error")) {
        const dot = river_dot(doc);
        console.log(dot);
        graph.renderDot(dot);
      }
    }
  }
}));

const input = editor(x => x, [
  history(),
  keymap({ "Mod-z": undo, "Mod-Shift-z": redo, "Mod-a": selectAll }),
  keymap(baseKeymap),
  river_parser.extension
]);
const output = editor(river, []);
append("input", input);
append("output", output);

const graph = d3
  .select("#graph")
  .graphviz()
  .renderDot(river_dot(start));
