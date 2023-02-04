type KeyInfo =
  | string
  | string[]
  | {
      label: string | string[];
      width?: number;
      offset?: number;
    };

export function Key({ keyInfo: key }: { keyInfo: KeyInfo }) {
  const label = Array.isArray(key)
    ? key
    : typeof key === "object"
    ? Array.isArray(key.label)
      ? key.label
      : [key.label]
    : [key];

  const width =
    typeof key === "object" && !Array.isArray(key) ? key.width || 3 : 3;

  const offset =
    typeof key === "object" && !Array.isArray(key) ? key.offset || 0 : 0;

  return (
    <button
      className="bg-gray-600 text-white flex flex-col justify-center"
      style={{
        width: `${width}rem`,
        marginLeft: `${offset}rem`,
      }}
    >
      {label.map((line) => (
        <span>{line}</span>
      ))}
    </button>
  );
}

export const PET: KeyInfo[][] = [
  ["@", "!", '"', "#", "$", "%", "'", "&", "\\", "(", ")", "<-", "[", "]"],
  [
    {
      label: ["OFF", "RVS"],
      width: 4,
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
      offset: 1,
      width: 4,
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
      width: 4.5,
    },
  ],
  [
    {
      label: "SHIFT",
      offset: 1,
      width: 6,
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
      width: 6,
    },
  ],
  [
    {
      label: "",
      width: 28.5,
      offset: 9.5,
    },
  ],
];

export default function Keyboard({ layout }: { layout: KeyInfo[][] }) {
  return (
    <div className="flex flex-col bg-yellow-50 p-12 gap-1">
      {layout.map((row, i) => (
        <div key={i} className="flex h-12 gap-1">
          {row.map((key, i) => (
            <Key keyInfo={key} key={i} />
          ))}
        </div>
      ))}
    </div>
  );
}
