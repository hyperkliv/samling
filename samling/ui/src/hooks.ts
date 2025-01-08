import { useEffect } from "react";

export function useTitle(parts: (string | null | undefined)[]) {
  useEffect(() => {
    const cleanedParts = parts.filter(
      (p) => p !== undefined && p !== null,
    ) as string[];
    /// Only change title if all parts are strings
    if (cleanedParts.length === parts.length) {
      const nonEmptyStrings = cleanedParts.filter((p) => p !== "");
      if (parts.length === 0) {
        document.title = "Samling";
      } else if (nonEmptyStrings.length > 0) {
        document.title = `Samling | ${nonEmptyStrings.join(" | ")}`;
      }
    }
  }, [parts]);
}
