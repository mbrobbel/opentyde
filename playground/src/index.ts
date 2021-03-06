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

const start = "Root<Group<Bits<64>,Rev<Dim<Group<Bits<64>,Rev<Bits<8>>>>>>>";

const editor = (fn, extensions) =>
  new EditorView({
    state: EditorState.create({
      doc: fn(start),
      extensions: extensions
    })
  });

const append = (name, editor) => {
  document.getElementById(name).appendChild(editor.dom);
};

const river_parser = ViewPlugin.create(view => ({
  update(update: ViewUpdate) {
    if (update.docChanged) {
      const doc = update.state.doc.toString();
      const parsed = river(doc);
      if (!parsed.startsWith("Error")) {
        graph.renderDot(river_dot(doc));
      } else {
        console.clear();
        console.warn(parsed);
        graph.renderDot("digraph {}");
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
append("input", input);

const graph = d3
  .select("#graph")
  .graphviz()
  .zoom(false)
  .renderDot(river_dot(start));
