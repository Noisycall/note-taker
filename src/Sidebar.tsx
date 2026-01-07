import { useEffect, useState } from "react";
import { store } from "@/utils.tsx";
import { invoke } from "@tauri-apps/api/core";

function FileTreeVis(
  props: { files: any; root?: boolean } = { files: {}, root: false },
) {
  let file = props.files;
  console.log(file);
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
        onClick={(evt) => {
          store.selectedFile = evt.currentTarget.id;
          console.log(store);
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
          return <FileTreeVis files={filer} />;
        })}
      </div>
    );
  }
}

export default function Sidebar() {
  let [files, setFiles] = useState({ files: [] } as any);
  useEffect(() => {
    (async () => {
      let val = await invoke("list_files");
      // @ts-ignore
      setFiles(val);
      console.log(val);
    })();
  }, []);
  return (
    <div id="sidebar" style={{ width: "10em" }}>
      <FileTreeVis files={files} root={true} />
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
    </div>
  );
}
