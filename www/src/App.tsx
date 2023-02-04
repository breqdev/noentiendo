import { Noentiendo, NoentiendoBuilder } from "noentiendo";
import { useEffect, useRef, useState } from "react";
import Emulator from "./Emulator";
import Keyboard, { C64, PET } from "./keyboards/Common";
import PetGraphicsKeyboard from "./keyboards/PetGraphics";
import roms from "./roms";

export default function App() {
  const [system, setSystem] = useState<"pet" | "vic" | "c64">("pet");
  const instance = useRef<any>();

  const layout = system === "pet" ? PET : C64;

  return (
    <div className="w-full h-full grid place-items-center bg-gray-400 p-4">
      <div className="flex flex-col items-center gap-4">
        <Emulator system={system} ref={instance} className="w-full" />
        <div className="flex gap-2">
          <button
            className="bg-white rounded px-2 py-1"
            onClick={() => setSystem("pet")}
          >
            PET
          </button>
          <button
            className="bg-white rounded px-2 py-1"
            onClick={() => setSystem("vic")}
          >
            VIC
          </button>
          <button
            className="bg-white rounded px-2 py-1"
            onClick={() => setSystem("c64")}
          >
            C64
          </button>
          <button
            className="bg-white rounded px-2 py-1"
            onClick={() => instance.current?.reset()}
          >
            Reset
          </button>
          <button
            className="bg-white rounded px-2 py-1"
            onClick={() =>
              instance.current?.dispatchKey(
                { Commodore: { Digit2: null } },
                true
              )
            }
          >
            Boop
          </button>
        </div>
      </div>
      <Keyboard layout={layout} dispatch={instance.current?.dispatchKey} />
    </div>
  );
}
