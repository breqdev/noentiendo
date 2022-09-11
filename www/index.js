import * as wasm from "noentiendo";
import bin from "../bin/capitalize.bin";

let bin64 = bin.split(";")[1].split(",")[1];

let rom = Uint8Array.from(atob(bin64), (c) => c.charCodeAt(0));

wasm.main(rom);
