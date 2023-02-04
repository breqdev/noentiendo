type KeyInfo =
  | string
  | string[]
  | {
      label: string | string[];
      width?: number;
      offset?: number;
    };

export function Key({
  keyInfo: key,
  onPress,
  onRelease,
}: {
  keyInfo: KeyInfo;
  onPress: () => void;
  onRelease: () => void;
}) {
  const label = Array.isArray(key)
    ? key
    : typeof key === "object"
    ? Array.isArray(key.label)
      ? key.label
      : [key.label]
    : [key];

  const width =
    typeof key === "object" && !Array.isArray(key) ? key.width || 1 : 1;

  const offset =
    typeof key === "object" && !Array.isArray(key) ? key.offset || 0 : 0;

  return (
    <div
      className="p-0.5 grid"
      style={{
        width: `${width * 3}rem`,
        marginLeft: `${offset * 3}rem`,
      }}
    >
      <button
        className="bg-gray-600 text-white flex flex-col items-stretch justify-center min-h-0 min-w-0"
        onMouseDown={() => onPress()}
        onMouseUp={() => onRelease()}
      >
        {label.map((line) => (
          <span className="text-center">{line}</span>
        ))}
      </button>
    </div>
  );
}

export const PET: KeyInfo[][] = [
  ["@", "!", '"', "#", "$", "%", "'", "&", "\\", "(", ")", "<-", "[", "]"],
  [
    {
      label: ["OFF", "RVS"],
      width: 1.5,
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
    "^",
    "<",
    ">",
  ],
  [
    {
      label: ["SHIFT", "LOCK"],
      offset: 1 / 3,
      width: 1.5,
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
    ":",
    ["RUN", "STOP"],
    {
      label: "RETURN",
      width: 1.5,
    },
  ],
  [
    {
      label: "SHIFT",
      offset: 1 / 3,
      width: 2,
    },
    "Z",
    "X",
    "C",
    "V",
    "B",
    "N",
    "M",
    ",",
    ";",
    "?",
    {
      label: "SHIFT",
      width: 2,
    },
  ],
  [
    {
      label: "",
      width: 8 + 5 / 6,
      offset: 3,
    },
  ],
];

export const C64: KeyInfo[][] = [
  [
    {
      label: "<-",
      offset: 0.5,
    },
    ["!", "1"],
    ['"', "2"],
    ["#", "3"],
    ["$", "4"],
    ["%", "5"],
    ["&", "6"],
    ["'", "7"],
    ["(", "8"],
    [")", "9"],
    ["", "0"],
    "+",
    "-",
    "£",
    ["CLR", "HOME"],
    ["INST", "DEL"],
  ],
  [
    {
      label: "CTRL",
      offset: 0.5,
      width: 1.5,
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
    "@",
    "*",
    "^",
    {
      label: "RESTORE",
      width: 1.5,
    },
  ],
  [
    ["RUN", "STOP"],
    ["SHIFT", "LOCK"],
    "A",
    "S",
    "D",
    "F",
    "G",
    "H",
    "J",
    "K",
    "L",
    ["(", ":"],
    [")", ";"],
    "=",
    {
      label: "RETURN",
      width: 2,
    },
  ],
  [
    "C=",
    {
      label: "SHIFT",
      width: 1.5,
    },
    "Z",
    "X",
    "C",
    "V",
    "B",
    "N",
    "M",
    ["<", ","],
    [">", "."],
    ["?", "/"],
    {
      label: "SHIFT",
      width: 1.5,
    },
    ["^", "CRSR", "v"],
    ["<-", "CRSR", "->"],
  ],
  [{ label: "", offset: 3, width: 8.5 }],
];

export default function Keyboard({
  layout,
  dispatch,
}: {
  layout: KeyInfo[][];
  dispatch: (key: any, down: boolean) => void;
}) {
  return (
    <div className="flex flex-col bg-yellow-50 p-12">
      {layout.map((row, i) => (
        <div key={i} className="flex h-12">
          {row.map((key, i) => (
            <Key
              keyInfo={key}
              key={i}
              onPress={() => {
                dispatch({ Commodore: { Digit2: null } }, true);
              }}
              onRelease={() => {
                dispatch({ Commodore: { Digit2: null } }, false);
              }}
            />
          ))}
        </div>
      ))}
    </div>
  );
}
