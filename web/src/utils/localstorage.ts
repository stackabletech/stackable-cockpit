import { Signal, createEffect, createRoot, createSignal } from 'solid-js';

function createStorageSignal(
  storage: Storage,
  name: string,
): Signal<string | undefined> {
  const signal = createSignal<string | undefined>(
    storage.getItem(name) || undefined,
  );
  // The session writer effect's lifecycle should be tied to that of the signal
  createRoot(() => {
    createEffect(() => {
      const value = signal[0]();
      if (value !== undefined) {
        storage.setItem(name, value);
      } else {
        storage.removeItem(name);
      }
    });
  });
  return signal;
}

export function createLocalStorageSignal(
  name: string,
): Signal<string | undefined> {
  return createStorageSignal(localStorage, name);
}
