/// <reference types="vite/client" />

declare module "noentiendo" {
  export class NoentiendoBuilder {
    constructor();

    // Set the canvas used for rendering.
    with_canvas(canvas: HTMLCanvasElement): NoentiendoBuilder;

    // Specify a library with the ROMs required for the system.
    with_roms(roms: Record<String, Uint8Array>): NoentiendoBuilder;

    // Specify the system to emulate.
    with_system(system: string): NoentiendoBuilder;

    build(): Noentiendo;
  }

  export class Noentiendo {
    constructor(builder: NoentiendoBuilder);

    // Stop emulating and clean up resources.
    close(): void;

    // Reset the emulated system and continue emulation.
    reset(): void;

    // Send a key event to the emulated system.
    // (Useful for JS-based on-screen keyboards.)
    dispatch_key(key: string, down: boolean): void;
  }
}

declare module "*.bin" {
  export default string;
}
