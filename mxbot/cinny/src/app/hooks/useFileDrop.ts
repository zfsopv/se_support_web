import { useCallback, DragEventHandler, RefObject } from 'react';

export const useFileDropHandler = (onDrop: (file: File[]) => void): DragEventHandler =>
  useCallback(
    (evt) => {
      evt.preventDefault();
    },
    [onDrop]
  );

export const useFileDropZone = (
  zoneRef: RefObject<HTMLElement>,
  onDrop: (file: File[]) => void
): boolean => false;
