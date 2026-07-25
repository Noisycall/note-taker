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
import { useRef } from "react";
let debounceTimeoutID = -1;
export default function App() {
  let { selectedFile, loaded, deserialize } = useSnapshot(store);
  let ref = useRef(null);
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
    <div
      ref={ref}
      style={{
        visibility: selectedFile ? "visible" : "hidden",
        maxWidth: "100vw",
      }}
    >
      <Plate
        editor={editor}
        onChange={({ editor }) => {
          if (!editor.api.isFocused() && !loaded && deserialize) {
            deserialize = false;
            return;
          }
          const markdownOutput = editor.api.markdown.serialize();
          clearTimeout(debounceTimeoutID);
          debounceTimeoutID = setTimeout(() => {
            console.info("Saving!", "content -> ", markdownOutput);
            invoke("update_note", {
              id: store.selectedFile,
              contents: markdownOutput,
            }).catch((err) => {
              console.error("Could not update note: ", err);
            });
          }, 250);
        }}
      >
        <EditorContainer>
          {loaded ? (
            <Editor placeholder="Type your amazing content here..." />
          ) : (
            <div>Loading...</div>
          )}
        </EditorContainer>
      </Plate>
    </div>
  );
}
