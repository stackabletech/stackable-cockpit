import { Signal, createEffect, createRoot, createSignal } from 'solid-js';

function createStorageSignal(
  storage: Storage,
  name: string,
): Signal<string | undefined> {
  const [value, setValue] = createSignal<string | undefined>(
    storage.getItem(name) || undefined,
  );
  // The session writer effect's lifecycle should be tied to that of the signal
  createRoot(() => {
    createEffect(() => {
      const currentValue = value();
      if (currentValue === undefined) {
        storage.removeItem(name);
      } else {
        storage.setItem(name, currentValue);
      }
    });
  });
  return [value, setValue];
}

export function createLocalStorageSignal(
  name: string,
): Signal<string | undefined> {
  return createStorageSignal(localStorage, name);
}
