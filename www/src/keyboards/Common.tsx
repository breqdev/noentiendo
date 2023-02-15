import { useMediaQuery } from "react-responsive";
import KeyInfo, { FullKeyInfo, KeyLayout } from "./mappings/keyinfo";

export function Key({
  keyInfo: key,
  onPress,
  onRelease,
}: {
  keyInfo: FullKeyInfo;
  onPress: () => void;
  onRelease: () => void;
}) {
  const { label, width, offset } = key;

  const scale = useMediaQuery({ query: "(min-width: 768px)" }) ? 3 : 1.25;

  return (
    <div
      className="p-0.5 grid"
      style={{
        width: `${width * scale}rem`,
        marginLeft: `${offset * scale}rem`,
      }}
    >
      <button
        className="bg-gray-600 text-white flex flex-col items-stretch justify-center min-h-0 min-w-0"
        onMouseDown={() => onPress()}
        onMouseUp={() => onRelease()}
        onTouchStart={() => onPress()}
        onTouchEnd={() => onRelease()}
      >
        {label.map((line) => (
          <span className="text-center">{line}</span>
        ))}
      </button>
    </div>
  );
}

export default function Keyboard({
  layout,
  dispatch,
}: {
  layout: KeyLayout;
  dispatch: (key: any, down: boolean) => void;
}) {
  return (
    <div className="flex flex-col sm:flex-row bg-yellow-50 min-w-0 p-2 gap-16">
      {layout.parts.map((part) => (
        <div className="flex flex-col">
          {part.keys.map((row, i) => (
            <div key={i} className="flex h-12">
              {row.map((key, i) => (
                <Key
                  keyInfo={key}
                  key={i}
                  onPress={() => {
                    console.log("press", layout.name, key.key, "ON");
                    dispatch({ [layout.name]: { [key.key]: null } }, true);
                  }}
                  onRelease={() => {
                    console.log("press", layout.name, key.key, "OFF");
                    dispatch({ [layout.name]: { [key.key]: null } }, false);
                  }}
                />
              ))}
            </div>
          ))}
        </div>
      ))}
    </div>
  );
}
