import * as wasm from "noentiendo";
// import basicBin from "../pet/basic.bin";
// import charBin from "../pet/char.bin";
// import editorBin from "../pet/editor.bin";
// import kernalBin from "../pet/kernal.bin";
import charBin from "../vic/char.bin";
import basicBin from "../vic/basic.bin";
import kernalBin from "../vic/kernal.bin";

document.getElementById("canvas").focus();

// let [basic, char, editor, kernal] = [basicBin, charBin, editorBin, kernalBin]
//   .map((x) => x.split(";")[1].split(",")[1])
//   .map((x) => Uint8Array.from(atob(x), (c) => c.charCodeAt(0)));

let [basic, char, kernal] = [basicBin, charBin, kernalBin]
  .map((x) => x.split(";")[1].split(",")[1])
  .map((x) => Uint8Array.from(atob(x), (c) => c.charCodeAt(0)));

// wasm.main(basic, char, editor, kernal);
wasm.main(basic, char, kernal);
