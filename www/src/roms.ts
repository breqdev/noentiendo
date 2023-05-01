import petBasicBin from "../../pet/basic.bin";
import petCharBin from "../../pet/char.bin";
import petEditorBin from "../../pet/editor.bin";
import petKernalBin from "../../pet/kernal.bin";
import vicCharBin from "../../vic/char.bin";
import vicBasicBin from "../../vic/basic.bin";
import vicKernalBin from "../../vic/kernal.bin";
import vicPacManBin from "../../vic/frogger.bin";
import c64BasicBin from "../../c64/basic.bin";
import c64CharBin from "../../c64/char.bin";
import c64KernalBin from "../../c64/kernal.bin";

const parseRom = async (url: string) => {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to fetch ${url}`);
  }
  const arrayBuffer = await response.arrayBuffer();
  return new Uint8Array(arrayBuffer);
};

const roms = {
  pet: {
    basic: await parseRom(petBasicBin),
    char: await parseRom(petCharBin),
    editor: await parseRom(petEditorBin),
    kernal: await parseRom(petKernalBin),
  },
  vic: {
    basic: await parseRom(vicBasicBin),
    char: await parseRom(vicCharBin),
    kernal: await parseRom(vicKernalBin),
    // cartridge: await parseRom(vicPacManBin),
  },
  c64: {
    basic: await parseRom(c64BasicBin),
    char: await parseRom(c64CharBin),
    kernal: await parseRom(c64KernalBin),
  },
};

export default roms;
