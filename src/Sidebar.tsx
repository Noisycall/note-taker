import { useEffect, useState } from "react";
import { store } from "@/utils.tsx";
import { invoke } from "@tauri-apps/api/core";
import { PlateEditor, useEditorRef } from "platejs/react";
import { MarkdownPlugin } from "@platejs/markdown";
import { useContextMenu } from "@/hooks/useContextMenu.ts";
import { ContextMenu } from "@/components/ContextMenu.tsx";

function FileTreeVis(props: {
  files: any;
  root?: boolean;
  editor: PlateEditor;
  handleContextMenu: any;
}) {
  let file = props.files;
  const padLeft = "10px";
  if (!file.is_dir) {
    return (
      <button
        style={{
          width: "100%",
          textAlign: "left",
          paddingLeft: padLeft,
          border: "solid 2px red",
        }}
        key={file.path_from_docs}
        id={file.path_from_docs}
        onClick={async (evt) => {
          console.log("isFallback", props.editor);
          console.log("clicked");
          store.selectedFile = evt.currentTarget.id;
          console.log(store);
          let val = (await invoke("get_file", {
            path: evt.currentTarget.id,
          })) as string;
          console.log(val);
          let markAPI = props.editor?.getApi(MarkdownPlugin);
          props.editor?.tf.setValue(markAPI?.markdown.deserialize(val));
        }}
        onContextMenu={(e) => {
          props.handleContextMenu(e, file.path_from_docs);
        }}
      >
        {file.name}
      </button>
    );
  } else {
    return (
      <div
        style={{
          width: "100%",
          paddingLeft: padLeft,
          border: "dashed 2px black",
        }}
      >
        {props.root ? "" : <div style={{ width: "100%" }}>{file.name}</div>}
        {file.files.map((filer: any) => {
          return (
            <FileTreeVis
              files={filer}
              editor={props.editor}
              handleContextMenu={props.handleContextMenu}
            />
          );
        })}
      </div>
    );
  }
}

export default function Sidebar() {
  let [files, setFiles] = useState({ files: [] } as any);
  let [random, setrandom] = useState(false);
  const { menuState, handleContextMenu, handleAction } = useContextMenu(() => {
    setrandom(!random);
  });
  let editor = useEditorRef();
  useEffect(() => {
    (async () => {
      let val = await invoke("list_files");
      // @ts-ignore
      setFiles(val);
    })();
  }, [random]);
  return (
    <div id="sidebar" style={{ width: "10em" }}>
      <FileTreeVis
        files={files}
        root={true}
        editor={editor}
        handleContextMenu={handleContextMenu}
      />
      <button
        style={{
          width: "100%",
          fontWeight: "bolder",
          fontSize: "2em",
          border: "solid 1px black",
        }}
        onClick={async () => {
          await invoke("create_new_file");
          setFiles(await invoke("list_files"));
        }}
      >
        +
      </button>
      {/* Render the menu only if open */}
      {menuState.isOpen && (
        <ContextMenu
          x={menuState.x}
          y={menuState.y}
          onAction={handleAction}
          onClose={() => {}} // Handled by hook
        />
      )}
    </div>
  );
}
