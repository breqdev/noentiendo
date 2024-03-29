# noentiendo

[![codecov](https://codecov.io/gh/breqdev/noentiendo/branch/main/graph/badge.svg?token=iZqBLp1tGJ)](https://codecov.io/gh/breqdev/noentiendo)
[![github actions](https://github.com/breqdev/noentiendo/actions/workflows/actions.yml/badge.svg)](https://github.com/breqdev/noentiendo/actions/workflows/actions.yml)
[![GitHub last commit](https://img.shields.io/github/last-commit/breqdev/noentiendo)](https://github.com/breqdev/noentiendo/)
[![demo status](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fnoentiendo.breq.dev%2F)](https://noentiendo.breq.dev/)
[![docs status](https://img.shields.io/website?label=docs&url=https%3A%2F%2Fnoentiendo.breq.dev%2Fdoc%2Flibnoentiendo%2F)](https://noentiendo.breq.dev/doc/libnoentiendo/)
![powered by rust](https://img.shields.io/badge/powered%20by-rust-orange?logo=rust)
![made with love](https://img.shields.io/badge/made%20with-%F0%9F%A4%8D-lightgrey)

_A modular framework for emulating retro computers._

<center>
<div align="center">

[Demo](https://noentiendo.breq.dev/) | [Docs](https://noentiendo.breq.dev/doc/libnoentiendo/) | [Repo](https://github.com/breqdev/noentiendo/) | [License](https://github.com/breqdev/noentiendo/blob/main/LICENSE.txt)

![](https://i.imgur.com/1KpsVcK.png)

</div>
</center>

`noentiendo` is a framework for retro emulation. It focuses in implementing small building blocks like the `6502`, `6520`, and `6560` chips, then stitches them together to emulate a variety of systems. It runs in text-mode, as a desktop GUI application, or on the web with WebAssembly.

<center>
<div align="center">

| Supported Systems | Roadmap      |
| ----------------- | ------------ |
| Commodore PET     | Commodore 64 |
| Commodore VIC-20  | Apple IIe    |
|                   | Nintendo NES |

| Supported Platforms             | Roadmap                 |
| ------------------------------- | ----------------------- |
| Desktop (Linux, macOS, Windows) | Android (Native)        |
| Web (via WebAssembly)           | iOS (Native)            |
|                                 | Embedded (e.g., RP2040) |

</div>
</center>

## Core Contributors

<center>
<div align="center">

| <img src="https://github.com/breqdev.png" width="150" /> | <img src="https://github.com/dillydally414.png" width="150" /> | <img src="https://github.com/nkizz.png" width="150" /> | <img src="https://github.com/ava-silver.png" width="150" /> |
| :------------------------------------------------------: | :------------------------------------------------------------: | :----------------------------------------------------: | :---------------------------------------------------------: |
|           [Brooke Chalmers](https://breq.dev/)           |        [Dillon Scott](https://dillydally414.github.io)         |           [Mia Kiesman](https://nkizz.com/)            |            [Ava Silver](https://avasilver.dev/)             |

</div>
</center>

## License

This program is free software, licensed under the AGPLv3 license. In short, this means that:

- You may use this software for free, for personal or commercial use
- You may make modifications to this software, but these changes must retain the AGPLv3 license
- You may distribute this software or your modified version, but you must provide the source code to users
- You may allow users to interact with this software over a network connection, but you must provide the source code to users

For full details, consult [LICENSE.txt](https://github.com/breqdev/noentiendo/blob/main/LICENSE.txt).

Note that some files in this repo, such as provided ROMs, are licensed under their own terms.
