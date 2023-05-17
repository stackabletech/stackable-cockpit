import { For, JSX, Show, createMemo, createSignal } from 'solid-js';

export interface DataTableColumn<T> {
  label: string;
  get: (x: T) => JSX.Element;
  /// The key value that this column should be sorted by
  /// true => use `get`
  /// undefined => this column is not sortable
  sortable?: ((x: T) => any) | true;
}
export interface DataTableProps<T> {
  columns: DataTableColumn<T>[];
  items: T[];
}
export function DataTable<T>(props: DataTableProps<T>): JSX.Element {
  const [sortComparator, setSortComparator] = createSignal<
    ((x: T, y: T) => number) | null
  >(null);
  const sortedItems = createMemo(() => {
    const currSortComparator = sortComparator();
    if (currSortComparator != null) {
      const items = [...props.items];
      items.sort(currSortComparator);
      return items;
    } else {
      return props.items;
    }
  });
  return (
    <>
      <table>
        <thead>
          <tr>
            <For each={props.columns}>
              {(col) => (
                <th>
                  <Show when={col.sortable} fallback={col.label}>
                    <a
                      href="#"
                      onClick={() =>
                        setSortComparator(() =>
                          col.sortable
                            ? keyComparator(
                                col.sortable === true ? col.get : col.sortable,
                              )
                            : null,
                        )
                      }
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
              <tr>
                <For each={props.columns}>
                  {(col) => <td>{col.get(item)}</td>}
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

function keyComparator<T>(key: (x: T) => any): (x: T, y: T) => number {
  return (x, y) => collator.compare(key(x), key(y));
}
