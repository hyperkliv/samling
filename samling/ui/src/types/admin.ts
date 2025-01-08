import {
  I18NString,
  ImageSummary,
  NestedColor,
  NestedColorSummary,
  NestedStyle,
  NestedStyleSummary,
} from "./api";

export function makeEditableStyle(
  style: NestedStyle | NestedStyleSummary,
  fullStyle: NestedStyleSummary,
): EditableStyle {
  const colorMap = new Map(style.colors.map((color) => [color.id, color]));
  return {
    id: style.id,
    name: style.name,
    number: style.number,
    isNew: (style as NestedStyle).is_new || false,
    colors: fullStyle.colors.map((fullColor) => {
      const color = colorMap.get(fullColor.id);
      return {
        id: fullColor.id,
        name: fullColor.name,
        number: fullColor.number,
        primaryImage: fullColor.primary_image,
        isNew: (color as NestedColor)?.is_new || false,
        enabled:
          (style.colors as NestedColorSummary[]).find(
            (enabledColor) => enabledColor.id === fullColor.id,
          ) !== undefined,
        sizes: fullColor.sizes.map((fullSize) => ({
          id: fullSize.id,
          number: fullSize.number,
          enabled:
            (style.colors as NestedColorSummary[])
              .find((enabledColor) => enabledColor.id === fullColor.id)
              ?.sizes.find((enabledSize) => enabledSize.id === fullSize.id) !==
            undefined,
        })),
      };
    }),
  };
}

export interface EditableSize {
  id: number;
  number: string;
  enabled: boolean;
}

export interface EditableColor {
  id: number;
  name: I18NString;
  number: string;
  primaryImage?: ImageSummary | null;
  sizes: EditableSize[];
  isNew: boolean;
}

export interface EditableStyle {
  id: number;
  name: I18NString;
  number: string;
  colors: EditableColor[];
  isNew: boolean;
}

export function transferIsNew(prev: EditableStyle, current: EditableStyle): EditableStyle {
  current.isNew = prev.isNew;
  prev.colors.forEach((prevColor) => {
    current.colors.forEach((currColor) => {
      if (currColor.id === prevColor.id) {
        currColor.isNew = prevColor.isNew;
      }
    })
  })
  return current;
}
