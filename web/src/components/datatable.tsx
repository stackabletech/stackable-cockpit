import { For, JSX, Show, createMemo, createSignal } from 'solid-js';

export interface DataTableColumn<T> {
  label: string;
  get: (x: T) => JSX.Element;
  sortBy?: (x: T) => string;
}

export interface DataTableProps<T> {
  columns: DataTableColumn<T>[];
  items: T[];
}

export function DataTable<T>(props: DataTableProps<T>): JSX.Element {
  const [sortComparator, setSortComparator] = createSignal<
    ((x: T, y: T) => number) | undefined
  >();
  
  const sortedItems = createMemo(() => {
    const currentSortComparator = sortComparator();
    if (currentSortComparator == undefined) {
      return props.items;
    } else {
      const items = [...props.items];
      items.sort(currentSortComparator);
      return items;
    }
  });
  return (
    <>
      <table class="font-sans border-collapse text-left w-full">
        <thead class="text-xs uppercase text-gray-400 bg-gray-700">
          <tr>
            <For each={props.columns}>
              {(col) => (
                <th class="px-4 py-3">
                  <Show when={col.sortBy} fallback={col.label}>
                    <a
                      href=""
                      class="text-gray-400"
                      onClick={(event) => {
                        event.preventDefault();
                        setSortComparator(() =>
                          col.sortBy ? keyComparator(col.sortBy) : undefined,
                        );
                      }}
                    >
                      {col.label}
                    </a>
                  </Show>
                </th>
              )}
            </For>
          </tr>
        </thead>
        <tbody>
          <For each={sortedItems()}>
            {(item) => (
              <tr class="bg-gray-800 border-b border-b-style-solid border-gray-700">
                <For each={props.columns}>
                  {(col) => (
                    <td class="px-4 py-3 font-medium text-gray-400">
                      {col.get(item)}
                    </td>
                  )}
                </For>
              </tr>
            )}
          </For>
        </tbody>
      </table>
    </>
  );
}

const collator = new Intl.Collator();

function keyComparator<T>(key: (x: T) => string): (x: T, y: T) => number {
  return (x, y) => collator.compare(key(x), key(y));
}
