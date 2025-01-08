export function dummyLink(): string {
  return "/DUMMY-LINK";
}

export function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

export function camelCaseToWords(s: string): string[] {
  return s.match(/[A-Za-z][a-z]*/g) || [];
}

export function camelCaseToCapitalized(s: string): string {
  return camelCaseToWords(s).map(capitalize).join(" ");
}

export function camelCaseToConstantCase(s: string): string {
  return camelCaseToWords(s)
    .map((w) => w.toUpperCase())
    .join("_");
}

function capitalize(s: string): string {
  return s.charAt(0).toUpperCase() + s.substring(1);
}
