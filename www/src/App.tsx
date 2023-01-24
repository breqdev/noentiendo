import { Noentiendo, NoentiendoBuilder } from "noentiendo";
import { useEffect, useRef } from "react";
import roms from "./roms";

export default function App() {
  const instance = useRef<Noentiendo>();
  const canvas = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    instance.current = new NoentiendoBuilder()
      .with_canvas(canvas.current!)
      .with_roms(roms)
      .with_system("pet")
      .build();

    return () => {
      instance.current?.close();
    };
  });

  return (
    <div>
      <canvas ref={canvas} />
    </div>
  );
}
