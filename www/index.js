import * as wasm from "noentiendo";
import petBasicBin from "../pet/basic.bin";
import petCharBin from "../pet/char.bin";
import petEditorBin from "../pet/editor.bin";
import petKernalBin from "../pet/kernal.bin";
import vicCharBin from "../vic/char.bin";
import vicBasicBin from "../vic/basic.bin";
import vicKernalBin from "../vic/kernal.bin";

document.getElementById("canvas").focus();

let [petBasic, petChar, petEditor, petKernal] = [
  petBasicBin,
  petCharBin,
  petEditorBin,
  petKernalBin,
]
  .map((x) => x.split(";")[1].split(",")[1])
  .map((x) => Uint8Array.from(atob(x), (c) => c.charCodeAt(0)));

let [vicBasic, vicChar, vicKernal] = [vicBasicBin, vicCharBin, vicKernalBin]
  .map((x) => x.split(";")[1].split(",")[1])
  .map((x) => Uint8Array.from(atob(x), (c) => c.charCodeAt(0)));

let roms = {
  pet: {
    basic: petBasic,
    char: petChar,
    editor: petEditor,
    kernal: petKernal,
  },
  vic: {
    basic: vicBasic,
    char: vicChar,
    kernal: vicKernal,
  },
};

wasm.main(roms, "pet");
