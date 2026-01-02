// @ts-ignore
import type { Value } from "platejs";
import "./App.css";
import { Plate, usePlateEditor } from "platejs/react";

import { Editor, EditorContainer } from "@/components/ui/editor";

import { AutoformatKit } from "@/components/editor/plugins/autoformat-kit.tsx";
import { MarkdownKit } from "@/components/editor/plugins/markdown-kit.tsx";
import { FixedToolbarKit } from "@/components/editor/plugins/fixed-toolbar-kit.tsx";
import { FloatingToolbarKit } from "@/components/editor/plugins/floating-toolbar-kit.tsx";
import {
  BlockquotePlugin,
  BoldPlugin,
  H1Plugin,
  H2Plugin,
  H3Plugin,
  ItalicPlugin,
  UnderlinePlugin,
} from "@platejs/basic-nodes/react";
import {
  H1Element,
  H2Element,
  H3Element,
} from "@/components/ui/heading-node.tsx";
import { BlockquoteElement } from "@/components/ui/blockquote-node.tsx";
import { ListKit } from "@/components/editor/plugins/list-kit.tsx";

const initialValue: Value = [
  {
    children: [{ text: "Title" }],
    type: "h3",
  },
  {
    children: [{ text: "This is a quote." }],
    type: "blockquote",
  },
  {
    children: [
      { text: "With some " },
      { bold: true, text: "bold" },
      { text: " text for emphasis!" },
    ],
    type: "p",
  },
];

export default function App() {
  const editor = usePlateEditor({
    plugins: [
      ...ListKit,
      BoldPlugin,
      ItalicPlugin,
      UnderlinePlugin,
      H1Plugin.withComponent(H1Element),
      H2Plugin.withComponent(H2Element),
      H3Plugin.withComponent(H3Element),
      BlockquotePlugin.withComponent(BlockquoteElement),
      ...AutoformatKit,
      ...MarkdownKit,
      ...FixedToolbarKit,
      ...FloatingToolbarKit,
    ],
    value: () => {
      const savedValue = localStorage.getItem("installation-react-demo");
      return savedValue ? JSON.parse(savedValue) : initialValue;
    },
  });

  return (
    <div>
      <Plate
        editor={editor}
        onChange={({ value }) => {
          const markdownOutput = editor.api.markdown.serialize();
          console.info(markdownOutput);
          localStorage.setItem(
            "installation-react-demo",
            JSON.stringify(value),
          );
        }}
      >
        <EditorContainer>
          <Editor placeholder="Type your amazing content here..." />
        </EditorContainer>
      </Plate>
    </div>
  );
}
