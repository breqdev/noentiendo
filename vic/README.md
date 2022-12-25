# VIC-20 ROMS

Copyright held by Commodore Business Machines, used without permission, standard caveats of emulation apply.
All ROMs pulled from [zimmers.net](http://www.zimmers.net/anonftp/pub/cbm/firmware/computers/vic20/).

| ROM          | Mapping         | Description                     | Source                                                                                                                 |
| ------------ | --------------- | ------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `basic.bin`  | `0xC000-0xDFFF` | Interpreter for Commodore BASIC | [`basic.901486-01`](http://www.zimmers.net/anonftp/pub/cbm/firmware/computers/vic20/basic.901486-01.bin)               |
| `char.bin`   | `0x8000-0x8FFF` | Character bitmaps for graphics  | [`characters.901460-03.bin`](http://www.zimmers.net/anonftp/pub/cbm/firmware/computers/vic20/characters.901460-03.bin) |
| `kernal.bin` | `0xE000-0xFFFF` | Commodore Kernal, NTSC edition  | [`kernal.901486-06.bin`](http://www.zimmers.net/anonftp/pub/cbm/firmware/computers/vic20/kernal.901486-06.bin)         |

# Cartridges

Cartridges are mapped from `0xA000` to `0xBFFF`. With `.prg` files, the leading `0xA0 0x00` header must be removed from the file with:

```bash
dd if=program.prg of=program.bin bs=2 skip=1
```

| Cartridge ROM   | Description                           | Source                                                                                                                 |
| --------------- | ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `flightsim.bin` | IFR flight simulator by Ron Wanttaja. | [`IFR (Flight Simulator).prg`](<http://www.zimmers.net/anonftp/pub/cbm/vic20/carts/8k/IFR%20(Flight%20Simulator).prg>) |
| `pacman.bin`    | Pac-Man clone by Robert Hurst         | [`quickman-rom.a0`](http://www.zimmers.net/anonftp/pub/cbm/vic20/carts/4k/quikman-rom.a0)                              |
| `avenger.bin`   | Space Invaders clone.                 | [`Avenger.prg`](http://www.zimmers.net/anonftp/pub/cbm/vic20/roms/8k/Avenger.prg)                                      |
| `frogger.bin`   | Frogger, by Parker Brothers.          | [`Frogger.prg`](http://www.zimmers.net/anonftp/pub/cbm/vic20/roms/8k/Frogger.prg)                                      |
| `qbert.bin`     | Q-Bert, by Parker Brothers.           | [`Q-Bert.prg`](http://www.zimmers.net/anonftp/pub/cbm/vic20/roms/8k/Q-Bert.prg)                                        |
