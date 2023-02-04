import { Noentiendo, NoentiendoBuilder } from "noentiendo";
import { forwardRef, useEffect, useImperativeHandle, useRef } from "react";
import roms from "./roms";

const Emulator = forwardRef(
  (
    {
      system,
      className,
    }: {
      system: "pet" | "vic" | "c64";
      className?: string;
    },
    ref
  ) => {
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

    useImperativeHandle(ref, () => {
      return {
        reset: () => {
          instance.current?.reset();
        },
        dispatchKey: (key: any, down: boolean) => {
          instance.current?.dispatch_key(key, down);
        },
      };
    });

    return <canvas ref={canvas} className={className} />;
  }
);

export default Emulator;
