import { t, Trans } from "@lingui/macro";
import { Dispatch, SetStateAction, useState } from "react";
import { classNames } from "../../utils";

interface ExistingImage {
  src: string;
  alt: string;
}
interface ImageInputProps {
  label: string;
  accept: string[];
  maxSizeMegaBytes: number;
  description: string;
  newImage: File | null;
  setNewImage: Dispatch<SetStateAction<File | null>>;
  existing?: ExistingImage | null;
}

function preventEventAndPropagation(event: React.DragEvent) {
  event.stopPropagation();
  event.preventDefault();
}

export default function ImageInput({
  label,
  accept,
  existing,
  maxSizeMegaBytes,
  description,
  newImage,
  setNewImage,
}: ImageInputProps) {
  const imgUrl = newImage ? URL.createObjectURL(newImage) : existing?.src;
  const anyImage = accept.length === 0 || accept.includes("image/*");
  const fullDescription =
    (anyImage
      ? t`Up to ${maxSizeMegaBytes} MB.`
      : t`${accept.join(", ")}, up to ${maxSizeMegaBytes} MB.`) +
    " " +
    description;
  const [dragging, setDragging] = useState(false);

  function onDragEnter(event: React.DragEvent) {
    setDragging(true);
    preventEventAndPropagation(event);
  }

  function onDragOver(event: React.DragEvent) {
    preventEventAndPropagation(event);
  }

  function onDragExit(event: React.DragEvent) {
    setDragging(false);
    preventEventAndPropagation(event);
  }

  function onDrop(event: React.DragEvent) {
    setDragging(false);
    preventEventAndPropagation(event);
    uploadFile(event.dataTransfer.files.item(0));
  }

  function uploadFile(file?: File | null) {
    if (!!file) {
      setNewImage(file);
    }
  }

  return (
    <>
      <label
        htmlFor="cover-photo"
        className="block text-sm font-medium text-gray-700 sm:mt-px sm:py-2"
      >
        {label}
      </label>
      <div className="mt-1 sm:col-span-2 sm:mt-0">
        <div
          className={classNames(
            dragging ? "border-indigo-500" : "border-gray-300",
            "flex max-w-lg justify-center rounded-md border-2 border-dashed border-gray-300 px-6 pt-5 pb-6",
          )}
          onDragEnter={onDragEnter}
          onDragOver={onDragOver}
          onDragExit={onDragExit}
          onDrop={onDrop}
        >
          <div className="space-y-1 text-center">
            {!!imgUrl ? (
              <img
                src={imgUrl}
                alt={newImage ? t`Image to upload` : t`Existing image`}
              />
            ) : (
              <svg
                className="mx-auto h-12 w-12 text-gray-400"
                stroke="currentColor"
                fill="none"
                viewBox="0 0 48 48"
                aria-hidden="true"
              >
                <path
                  d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02"
                  strokeWidth={2}
                  strokeLinecap="round"
                  strokeLinejoin="round"
                />
              </svg>
            )}
            <div className="flex text-sm text-gray-600">
              <label
                htmlFor="image-upload"
                className="relative cursor-pointer rounded-md bg-white font-medium text-indigo-600 focus-within:outline-none focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 hover:text-indigo-500"
              >
                <Trans>Upload an image</Trans>
                <input
                  id="image-upload"
                  name="image-upload"
                  type="file"
                  onChange={(e) => uploadFile(e.target.files?.item(0))}
                  className="sr-only"
                  accept={accept.join(",")}
                />
              </label>
              <p className="pl-1">
                <Trans>or drag and drop.</Trans>
              </p>
            </div>
            <p className="text-xs text-gray-500">{fullDescription}</p>
          </div>
        </div>
      </div>
    </>
  );
}
