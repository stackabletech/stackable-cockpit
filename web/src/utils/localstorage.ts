import { Signal, createEffect, createRoot, createSignal } from 'solid-js';
import { Option, someIfNotNull } from '../types';

function createStorageSignal(
  storage: Storage,
  name: string,
): Signal<Option<string>> {
  const [value, setValue] = createSignal<Option<string>>(
    someIfNotNull(storage.getItem(name)),
  );
  // The session writer effect's lifecycle should be tied to that of the signal
  createRoot(() => {
    createEffect(() => {
      value().match({
        none: () => storage.removeItem(name),
        some: (currentValue) => storage.setItem(name, currentValue),
      });
    });
  });
  return [value, setValue];
}

export function createLocalStorageSignal(name: string): Signal<Option<string>> {
  return createStorageSignal(localStorage, name);
}
