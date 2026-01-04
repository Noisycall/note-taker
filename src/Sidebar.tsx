import { useEffect, useState } from "react";
import { store } from "@/utils.tsx";
export default function Sidebar() {
  let [files, setFiles] = useState([]);
  useEffect(() => {
    (async () => {
      invoke("list_files");
      invoke("create_new_file");
    })();
  }, []);
  return (
    <div id="sidebar" style={{ width: "10em" }}>
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
      <button className="">+</button>
    </div>
  );
}
