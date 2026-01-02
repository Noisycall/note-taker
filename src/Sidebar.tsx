import { useEffect } from "react";
import { store } from "@/utils.tsx";

export default function Sidebar() {
  let files = ["file1", "file2", "file3"];
  useEffect(() => {}, []);
  return (
    <div id="sidebar">
      {files.map((file) => {
        return (
          <button
            style={{ width: "100%" }}
            id={file}
            onClick={(evt) => {
              store.selectedFile = evt.currentTarget.id;
              console.log(store);
            }}
          >
            {file}
          </button>
        );
      })}
    </div>
  );
}
