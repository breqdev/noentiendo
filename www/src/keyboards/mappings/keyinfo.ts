export type FullKeyInfo = {
  label: string[];
  key: string;
  width: number;
  offset: number;
  toggle: boolean;
};

type KeyInfo =
  | string // string == label == key
  | [string, string] // [label, key]
  | [string[], string] // [label, key]
  | {
      label: string | string[];
      key: string;
      width?: number;
      offset?: number;
      toggle?: boolean;
    };

export default KeyInfo;

function forceArray<T>(value: T | T[]): T[] {
  return Array.isArray(value) ? value : [value];
}

export function normalizeKeyInfo(info: KeyInfo): FullKeyInfo {
  if (Array.isArray(info)) {
    return {
      label: forceArray(info[0]),
      key: info[1],
      width: 1,
      offset: 0,
      toggle: false,
    };
  }

  if (typeof info === "object") {
    return {
      label: forceArray(info.label),
      key: info.key,
      width: info.width || 1,
      offset: info.offset || 0,
      toggle: info.toggle || false,
    };
  }

  if (typeof info === "string") {
    return {
      label: [info],
      key: info,
      width: 1,
      offset: 0,
      toggle: false,
    };
  }

  throw new Error("Invalid key info");
}

export type KeyboardPart = {
  name: string;
  keys: FullKeyInfo[][];
};

export type KeyLayout = {
  name: string;
  parts: KeyboardPart[];
};

export function normalizeLayout(
  name: string,
  parts: { name: string; keys: KeyInfo[][] }[]
): KeyLayout {
  return {
    name,
    parts: parts.map((part) => ({
      name: part.name,
      keys: part.keys.map((row) => row.map(normalizeKeyInfo)),
    })),
  };
}
