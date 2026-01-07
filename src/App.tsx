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
import { invoke } from "@tauri-apps/api/core";
import { store } from "@/utils.tsx";
import { useSnapshot } from "valtio/react";

export default function App() {
  let { selectedFile } = useSnapshot(store);
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

    onReady: ({ value }) => {
      console.info("Editor ready with loaded value:", value);
    },
  });

  return (
    <div style={{ visibility: selectedFile ? "visible" : "hidden" }}>
      <Plate
        editor={editor}
        onChange={({ editor }) => {
          const markdownOutput = editor.api.markdown.serialize();
          console.info(markdownOutput);
          invoke("set_file", {
            path: store.selectedFile,
            value: markdownOutput,
          });
        }}
      >
        <EditorContainer>
          <Editor placeholder="Type your amazing content here..." />
        </EditorContainer>
      </Plate>
    </div>
  );
}
