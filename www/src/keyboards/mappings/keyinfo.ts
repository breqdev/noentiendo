export type FullKeyInfo = {
  label: string[];
  key: string;
  width: number;
  offset: number;
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
    };
  }

  if (typeof info === "object") {
    return {
      label: forceArray(info.label),
      key: info.key,
      width: info.width || 1,
      offset: info.offset || 0,
    };
  }

  if (typeof info === "string") {
    return {
      label: [info],
      key: info,
      width: 1,
      offset: 0,
    };
  }

  throw new Error("Invalid key info");
}

export type KeyLayout = {
  name: string;
  keys: FullKeyInfo[][];
};

export function normalizeLayout(name: string, layout: KeyInfo[][]): KeyLayout {
  return {
    name,
    keys: layout.map((row) => row.map(normalizeKeyInfo)),
  };
}
