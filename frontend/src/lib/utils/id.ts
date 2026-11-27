/**
 * Snowflake ids are 64-bit integers serialized as strings. Never convert them
 * to `number`: values above `Number.MAX_SAFE_INTEGER` would lose precision.
 */
export function compareIds(a: string, b: string): number {
  try {
    const left = BigInt(a);
    const right = BigInt(b);

    if (left < right) return -1;
    if (left > right) return 1;
    return 0;
  } catch {
    return a.localeCompare(b);
  }
}

export function idIsGreaterOrEqual(a: string, b: string): boolean {
  return compareIds(a, b) >= 0;
}

export function nextId(id: string): string {
  try {
    return (BigInt(id) + 1n).toString();
  } catch {
    return id;
  }
}

export function maxId(ids: (string | null | undefined)[]): string | null {
  let max: string | null = null;

  for (const id of ids) {
    if (!id) continue;
    if (max === null || compareIds(id, max) > 0) max = id;
  }

  return max;
}
