import { proxy } from "valtio";

export const store = proxy({
  selectedFile: "",
  loaded: false,
  deserialize: false,
});
