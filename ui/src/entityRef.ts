// NOTE: We're not using any "Ref" from types/api.ts as they are duplicated for each
//       type.
export interface Ref {
  id?: number | string;
  external_id?: string;
  slug?: string;
}

export function refIsValid(ref: Ref): boolean {
  return (
    typeof ref.id === "number" ||
    typeof ref.id === "string" ||
    typeof ref.external_id === "string" ||
    typeof ref.slug === "string"
  );
}

export function entityRefToPath(ref: Ref): string {
  if (!!ref.id) {
    return `id:${ref.id}`;
  } else if (!!ref.slug) {
    return `slug:${ref.slug}`;
  } else if (!!ref.external_id) {
    return `external_id:${ref.external_id}`;
  } else {
    throw Error(`Invalid ref: ${JSON.stringify(ref)}`);
  }
}
