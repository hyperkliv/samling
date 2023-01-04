import { ImageSummary, NestedStyle } from "./types/api";

export type CloudflareImageSize = "original" | "medium" | "thumbnail";

// TODO: This should be handled at the API layer
export function cloudflareImageUrl(
  imageUrl: string,
  newSize: CloudflareImageSize,
): string {
  return imageUrl.replace("/original", `/${newSize}`);
}

export function getStyleDisplayImage(style: NestedStyle): ImageSummary | null {
  return style.colors.reduce((ret, color) => {
    if (ret === null) {
      return color.images[0] || null;
    } else {
      return ret;
    }
  }, null as ImageSummary | null);
}
