import "./Homepage.css";
import App from "@/App.tsx";
import Sidebar from "@/Sidebar.tsx";
import { PlateController } from "platejs/react";
export default function Homepage() {
  return (
    <PlateController>
      <div id="main">
        <Sidebar />
        <App />
        <div />
        <div />
      </div>
    </PlateController>
  );
}
