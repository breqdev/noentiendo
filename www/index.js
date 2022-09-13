import * as wasm from "noentiendo";
import basicBin from "../pet/basic.bin";
import charBin from "../pet/char.bin";
import editorBin from "../pet/editor.bin";
import kernalBin from "../pet/kernal.bin";

console.log("hiii");
console.log("focusing canvas");
document.getElementById("canvas").focus();

let [basic, char, editor, kernal] = [basicBin, charBin, editorBin, kernalBin]
  .map((x) => x.split(";")[1].split(",")[1])
  .map((x) => Uint8Array.from(atob(x), (c) => c.charCodeAt(0)));

wasm.main(basic, char, editor, kernal);
