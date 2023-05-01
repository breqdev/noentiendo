import KeyInfo, { KeyLayout, normalizeLayout } from "./keyinfo";

function digit(digit: number) {
  return {
    label: digit.toString(),
    key: `Num${digit}`,
  };
}

const PET: KeyLayout = normalizeLayout("CommodorePet", [
  {
    name: "main",
    keys: [
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
      ],
      [
        {
          label: ["SHIFT", "LOCK"],
          offset: 1 / 3,
          width: 1.5,
          key: "ShiftLock",
          toggle: true,
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
      ],
      [
        {
          label: "SHIFT",
          offset: 1 / 3,
          width: 2,
          key: "LShift",
          toggle: true,
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
          toggle: true,
        },
      ],
      [
        {
          label: "",
          width: 8 + 5 / 6,
          offset: 3,
          key: "Space",
        },
      ],
    ],
  },
  {
    name: "numpad",
    keys: [
      [
        [["CLR", "HOME"], "ClrHome"],
        [["^", "CRSR", "v"], "CursorUpDown"],
        [["<-", "CRSR", "->"], "CursorLeftRight"],
        [["INST", "DEL"], "InsertDelete"],
      ],
      [digit(7), digit(8), digit(9), ["/", "NumDivide"]],
      [digit(4), digit(5), digit(6), ["*", "NumMultiply"]],
      [digit(1), digit(2), digit(3), ["+", "NumPlus"]],
      [digit(0), [".", "NumPeriod"], ["-", "NumMinus"], ["=", "NumEquals"]],
    ],
  },
]);

export default PET;
