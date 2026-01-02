import "./Homepage.css";
import { store } from "@/utils.tsx";
import App from "@/App.tsx";
import Sidebar from "@/Sidebar.tsx";
import { useSnapshot } from "valtio";

export default function Homepage() {
  let selectedFile = useSnapshot(store);
  return (
    <div id="main">
      <Sidebar />
      {selectedFile ? <App /> : ""}
      <div />
      <div />
    </div>
  );
}
