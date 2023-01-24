import { Noentiendo, NoentiendoBuilder } from "noentiendo";
import { useEffect, useRef } from "react";
import roms from "./roms";

export default function Emulator({
  system,
  className,
}: {
  system: "pet" | "vic" | "c64";
  className?: string;
}) {
  const instance = useRef<Noentiendo>();
  const canvas = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    instance.current = new NoentiendoBuilder()
      .with_canvas(canvas.current!)
      .with_roms(roms)
      .with_system(system)
      .build();

    return () => {
      instance.current?.close();
    };
  }, [system]);

  return <canvas ref={canvas} className={className} />;
}
