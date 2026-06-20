import { useEffect, useState } from "react";
import { store } from "@/utils.tsx";
import { invoke } from "@tauri-apps/api/core";
import { PlateEditor, useEditorRef } from "platejs/react";
import { MarkdownPlugin } from "@platejs/markdown";
import { useContextMenu } from "@/hooks/useContextMenu.ts";
import { ContextMenu } from "@/components/ContextMenu.tsx";

function FileTreeVis(props: {
  files?: NoteTree;
  root?: boolean;
  file?: NoteTreeEntry;
  fileName?: string;
  editor: PlateEditor;
  handleContextMenu: any;
}) {
  let files = props.files;
  let file = props.file;
  const padLeft = "10px";
  if (file && file.href) {
    return (
      <button
        style={{
          width: "100%",
          textAlign: "left",
          paddingLeft: padLeft,
          border: "solid 2px red",
        }}
        key={file.href}
        id={file.href}
        onClick={async (evt) => {
          console.log("isFallback", props.editor);
          console.log("clicked");
          store.selectedFile = evt.currentTarget.id;
          console.log(store);
          let val = (await get_note(file)).contents;
          console.log(val);
          let markAPI = props.editor?.getApi(MarkdownPlugin);
          props.editor?.tf.setValue(markAPI?.markdown.deserialize(val));
        }}
        onContextMenu={(e) => {
          props.handleContextMenu(e, file.href);
        }}
      >
        {props.fileName}
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
        {props.root ? (
          ""
        ) : (
          <div style={{ width: "100%" }}>{props.fileName}</div>
        )}
        {files &&
          Object.entries(files).map((entry) => {
            return (
              <FileTreeVis
                //@ts-ignore
                files={entry[1].href ? null : entry[1]}
                //@ts-ignore
                file={entry[1].href ? entry[1] : null}
                fileName={decodeURI(entry[0]).split(".md")[0]}
                editor={props.editor}
                handleContextMenu={props.handleContextMenu}
              />
            );
          })}
      </div>
    );
  }
}

type NoteTreeEntry = {
  content_length: number;
  content_type: string;
  href: string;
  last_modified: string;
  tag: string;
};

interface NoteTree {
  [note_name: string]: NoteTreeEntry | NoteTree;
}

interface Note {
  id: string;
  contents: string;
}

const get_note_tree = async (): Promise<NoteTree> => {
  return JSON.parse(await invoke("get_webdav_notes_tree")) as NoteTree;
};
const get_note = async (val: NoteTreeEntry): Promise<Note> => {
  return await invoke("get_note", {
    id: val.href,
  });
};
export default function Sidebar() {
  let [files, setFiles] = useState({} as NoteTree);
  let [random, setRandom] = useState(false);
  const { menuState, handleContextMenu, handleAction } = useContextMenu(() => {
    setRandom(!random);
  });
  let editor = useEditorRef();
  useEffect(() => {
    (async () => {
      let val = await get_note_tree();
      // @ts-ignore
      setFiles(val);
    })();
  }, [random]);
  return (
    <div id="sidebar" style={{ width: "15em" }}>
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
          setFiles(await get_note_tree());
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
