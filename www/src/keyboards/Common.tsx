import { useState } from "react";
import { useMediaQuery } from "react-responsive";
import KeyInfo, {
  FullKeyInfo,
  KeyLayout,
  KeyboardPart,
} from "./mappings/keyinfo";

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

  const scale = useMediaQuery({ query: "(min-width: 768px)" }) ? 3 : 1.2;

  return (
    <div
      className="p-0.5 grid overflow-hidden"
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

function KeyboardPartView({
  part,
  onKey,
}: {
  part: KeyboardPart;
  onKey: (keyname: string, state: boolean) => void;
}) {
  return (
    <div className="flex flex-col">
      {part.keys.map((row, i) => (
        <div key={i} className="flex h-12">
          {row.map((key, i) => (
            <Key
              keyInfo={key}
              key={i}
              onPress={() => {
                // dispatch({ [layout.name]: { [key.key]: null } }, true);
                onKey(key.key, true);
              }}
              onRelease={() => {
                // dispatch({ [layout.name]: { [key.key]: null } }, false);
                onKey(key.key, false);
              }}
            />
          ))}
        </div>
      ))}
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
  const mobile = useMediaQuery({ query: "(max-width: 768px)" });

  const [tab, setTab] = useState(0);

  const handleKey = (key: string, down: boolean) => {
    dispatch({ [layout.name]: { [key]: null } }, down);
  };

  return (
    <>
      <div className="flex flex-col sm:flex-row bg-yellow-50 min-w-0 p-2 gap-16">
        {mobile ? (
          <KeyboardPartView part={layout.parts[tab]} onKey={handleKey} />
        ) : (
          layout.parts.map((part) => (
            <KeyboardPartView part={part} onKey={handleKey} />
          ))
        )}
      </div>
      {mobile && (
        <div className="flex flex-row gap-2">
          {layout.parts.map((part, i) => (
            <button
              key={i}
              onClick={() => setTab(i)}
              className="bg-gray-200 rounded py-1 px-2"
            >
              {part.name}
            </button>
          ))}
        </div>
      )}
    </>
  );
}
