import KeyInfo, { KeyLayout, normalizeLayout } from "./keyinfo";

function digit(digit: number, symbol: string): KeyInfo {
  return {
    label: [symbol, digit.toString()],
    key: `Digit${digit}`,
  };
}

const C64: KeyLayout = normalizeLayout("Commodore", [
  [
    {
      label: "<-",
      offset: 0.5,
      key: "LeftArrow",
    },
    digit(1, "!"),
    digit(2, '"'),
    digit(3, "#"),
    digit(4, "$"),
    digit(5, "%"),
    digit(6, "&"),
    digit(7, "'"),
    digit(8, "("),
    digit(9, ")"),
    digit(0, ""),
    ["+", "Plus"],
    ["-", "Minus"],
    ["£", "Pound"],
    [["CLR", "HOME"], "ClrHome"],
    [["INST", "DEL"], "InsertDelete"],
    {
      label: "F1",
      offset: 1.5,
      width: 1 + 2 / 3,
      key: "F1",
    },
  ],
  [
    {
      label: "CTRL",
      offset: 0.5,
      width: 1.5,
      key: "Control",
    },
    "Q",
    "W",
    "E",
    "R",
    "T",
    "Y",
    "U",
    "I",
    "O",
    "P",
    ["@", "At"],
    ["*", "Asterisk"],
    ["^", "UpArrow"],
    {
      label: "RESTORE",
      width: 1.5,
      key: "Restore",
    },
    {
      label: "F3",
      offset: 1.5,
      width: 1 + 2 / 3,
      key: "F3",
    },
  ],
  [
    [["RUN", "STOP"], "RunStop"],
    [["SHIFT", "LOCK"], "ShiftLock"],
    "A",
    "S",
    "D",
    "F",
    "G",
    "H",
    "J",
    "K",
    "L",
    [["(", ":"], "Colon"],
    [[")", ";"], "Semicolon"],
    ["=", "Equals"],
    {
      label: "RETURN",
      width: 2,
      key: "Return",
    },
    {
      label: "F5",
      offset: 2,
      width: 1 + 2 / 3,
      key: "F5",
    },
  ],
  [
    ["C=", "Commodore"],
    {
      label: "SHIFT",
      width: 1.5,
      key: "LShift",
    },
    "Z",
    "X",
    "C",
    "V",
    "B",
    "N",
    "M",
    [["<", ","], "Comma"],
    [[">", "."], "Period"],
    [["?", "/"], "Slash"],
    {
      label: "SHIFT",
      width: 1.5,
      key: "RShift",
    },
    [["^", "CRSR", "v"], "CursorUpDown"],
    [["<-", "CRSR", "->"], "CursorLeftRight"],
    {
      label: "F7",
      offset: 2,
      width: 1 + 2 / 3,
      key: "F7",
    },
  ],
  [
    {
      label: "",
      offset: 3,
      width: 8.5,
      key: "Space",
    },
  ],
]);

export default C64;
