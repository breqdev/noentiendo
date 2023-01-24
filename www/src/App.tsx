import { Noentiendo, NoentiendoBuilder } from "noentiendo";
import { useEffect, useRef, useState } from "react";
import Emulator from "./Emulator";
import roms from "./roms";

export default function App() {
  let [system, setSystem] = useState<"pet" | "vic" | "c64">("pet");
  const instance = useRef<any>();

  return (
    <div className="w-full h-full grid place-items-center bg-gray-400">
      <div className="flex flex-col items-center gap-4">
        <Emulator system={system} ref={instance} />
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
        </div>
      </div>
    </div>
  );
}
