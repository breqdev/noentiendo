import KeyInfo, { KeyLayout, normalizeLayout } from "./keyinfo";

function digit(digit: number) {
  return {
    label: digit.toString(),
    key: `Num${digit}`,
  };
}

const PET: KeyLayout = normalizeLayout("CommodorePet", [
  [
    ["@", "At"],
    ["!", "Exclamation"],
    ['"', "DoubleQuote"],
    ["#", "Hash"],
    ["$", "Dollar"],
    ["%", "Percent"],
    ["'", "Apostrophe"],
    ["&", "Ampersand"],
    ["\\", "Backslash"],
    ["(", "LeftParen"],
    [")", "RightParen"],
    ["<-", "LeftArrow"],
    ["[", "LeftBracket"],
    ["]", "RightBracket"],
    {
      label: ["CLR", "HOME"],
      offset: 1.5,
      key: "ClrHome",
    },
    {
      label: ["^", "CRSR", "v"],
      key: "CursorUpDown",
    },
    {
      label: ["<-", "CRSR", "->"],
      key: "CursorLeftRight",
    },
    {
      label: ["INST", "DEL"],
      key: "InsertDelete",
    },
  ],
  [
    {
      label: ["OFF", "RVS"],
      width: 1.5,
      key: "Reverse",
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
    ["^", "UpArrow"],
    ["<", "LessThan"],
    [">", "GreaterThan"],
    {
      offset: 1,
      ...digit(7),
    },
    digit(8),
    digit(9),
    {
      label: "/",
      key: "NumDivide",
    },
  ],
  [
    {
      label: ["SHIFT", "LOCK"],
      offset: 1 / 3,
      width: 1.5,
      key: "ShiftLock",
    },
    "A",
    "S",
    "D",
    "F",
    "G",
    "H",
    "J",
    "K",
    "L",
    [":", "Colon"],
    ["RUN", "STOP"],
    {
      label: "RETURN",
      width: 1.5,
      key: "Return",
    },
    {
      offset: 1 + 1 / 6,
      ...digit(4),
    },
    digit(5),
    digit(6),
    {
      label: "*",
      key: "NumMultiply",
    },
  ],
  [
    {
      label: "SHIFT",
      offset: 1 / 3,
      width: 2,
      key: "LShift",
    },
    "Z",
    "X",
    "C",
    "V",
    "B",
    "N",
    "M",
    [",", "Comma"],
    [";", "Semicolon"],
    ["?", "Question"],
    {
      label: "SHIFT",
      width: 2,
      key: "RShift",
    },
    {
      offset: 1 + 1 / 6,
      ...digit(1),
    },
    digit(2),
    digit(3),
    {
      label: "+",
      key: "NumPlus",
    },
  ],
  [
    {
      label: "",
      width: 8 + 5 / 6,
      offset: 3,
      key: "Space",
    },
    {
      offset: 3 + 2 / 3,
      ...digit(0),
    },
    {
      label: ".",
      key: "NumPeriod",
    },
    {
      label: "-",
      key: "NumMinus",
    },
    {
      label: "=",
      key: "NumEquals",
    },
  ],
]);

export default PET;
