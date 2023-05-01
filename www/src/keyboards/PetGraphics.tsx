type KeyInfo =
  | string
  | string[]
  | null
  | {
      label: string | string[];
      color?: "gray" | "red" | "blue";
      type?: "horizontal" | "vertical";
      cols?: number;
      rows?: number;
    };

function Key({ keyInfo: key }: { keyInfo: KeyInfo }) {
  if (key === null) {
    return null;
  }

  const colors = {
    gray: "bg-gray-300",
    red: "bg-red-300",
    blue: "bg-blue-300",
  };

  const color =
    typeof key === "object" && !Array.isArray(key) && key !== null
      ? key.color || "gray"
      : "gray";

  const label_or_labels =
    typeof key === "object" && !Array.isArray(key) && key !== null
      ? key.label
      : key;

  const labels = Array.isArray(label_or_labels)
    ? label_or_labels
    : [label_or_labels];

  if (typeof key === "object" && !Array.isArray(key) && key.type) {
    if (key.type === "horizontal") {
      return (
        <button className="h-16 bg-white p-2 flex col-span-2">
          <div
            className={
              "rounded-xl flex-grow flex flex-col font-bold items-center " +
              colors[color]
            }
          >
            {labels.map((l) => (
              <span>{l}</span>
            ))}
          </div>
        </button>
      );
    } else if (key.type === "vertical") {
      return (
        <button className="w-16 bg-white p-2 flex row-span-2">
          <div
            className={
              "rounded-xl flex-grow flex flex-col font-bold items-center " +
              colors[color]
            }
          >
            {labels.map((l) => (
              <span
                style={{
                  writingMode: "vertical-lr",
                  textOrientation: "upright",
                }}
              >
                {l}
              </span>
            ))}
          </div>
        </button>
      );
    }
  }

  return (
    <button className="w-16 h-16 bg-white p-2 flex">
      <div
        className={
          "rounded-xl flex-grow flex flex-col font-bold items-center " +
          colors[color]
        }
      >
        {labels.map((l) => (
          <span>{l}</span>
        ))}
      </div>
    </button>
  );
}

const KEYS: KeyInfo[][] = [
  [
    ["", "!"],
    ["", '"'],
    ["", "#"],
    ["", "$"],
    ["", "%"],
    ["", "'"],
    ["", "&"],
    ["", "\\"],
    ["", "("],
    ["", ")"],
    ["", "<-"],
  ],
  [
    ["", "Q"],
    ["", "W"],
    ["", "E"],
    ["", "R"],
    ["", "T"],
    ["", "Y"],
    ["", "U"],
    ["", "I"],
    ["", "O"],
    ["", "P"],
    ["", "^"],
  ],
  [
    ["", "A"],
    ["", "S"],
    ["", "D"],
    ["", "F"],
    ["", "G"],
    ["", "H"],
    ["", "J"],
    ["", "K"],
    ["", "L"],
    ["", ":"],
    {
      label: "RETURN",
      color: "red",
      type: "vertical",
      rows: 2,
    },
  ],
  [
    ["", "Z"],
    ["", "X"],
    ["", "C"],
    ["", "V"],
    ["", "B"],
    ["", "N"],
    ["", "M"],
    ["", ","],
    ["", ";"],
    ["", "?"],
    null,
  ],
  [
    {
      label: "SHIFT",
      color: "red",
    },
    {
      label: ["OFF", "RVS"],
      color: "blue",
    },
    ["", "@"],
    ["", "["],
    ["", "]"],
    {
      label: "SPACE",
      color: "red",
      type: "horizontal",
      cols: 2,
    },
    null,
    ["", "<"],
    ["", ">"],
    {
      label: ["RUN", "STOP"],
      color: "blue",
    },
    {
      label: "SHIFT",
      color: "red",
    },
  ],
];

const NUMPAD: KeyInfo[][] = [
  [
    {
      label: ["CLR", "HOME"],
      color: "red",
    },
    {
      label: ["^", "CRSR", "v"],
      color: "blue",
    },
    {
      label: ["<-", "CRSR", "->"],
      color: "blue",
    },
    {
      label: ["INS", "DEL"],
      color: "red",
    },
  ],
  [
    ["", "7"],
    ["", "8"],
    ["", "9"],
    ["", "/"],
  ],
  [
    ["", "4"],
    ["", "5"],
    ["", "6"],
    ["", "*"],
  ],
  [
    ["", "1"],
    ["", "2"],
    ["", "3"],
    ["", "+"],
  ],
  [
    ["", "0"],
    ["", "."],
    ["", "-"],
    ["", "="],
  ],
];

export default function PetGraphicsKeyboard() {
  return (
    <div className="flex gap-24 bg-gray-100 p-8">
      <div className="grid grid-cols-[repeat(11,minmax(0,1fr))] grid-rows-5 gap-1 p-2 bg-black">
        {Array(11 * 5)
          .fill(0)
          .map((_, i) => {
            let row = Math.floor(i / 11);
            let col = i % 11;
            let key = KEYS[row][col];
            return <Key key={i} keyInfo={key} />;
          })}
      </div>
      <div className="grid grid-cols-4 grid-rows-16 gap-1 p-2 bg-black">
        {Array(4 * 5)
          .fill(0)
          .map((_, i) => {
            let row = Math.floor(i / 4);
            let col = i % 4;
            let key = NUMPAD[row][col];
            return <Key key={i} keyInfo={key} />;
          })}
      </div>
    </div>
  );
}
