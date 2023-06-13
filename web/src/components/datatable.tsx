import { For, JSX, Show, createMemo, createSignal } from 'solid-js';
import { Button } from './button';
import { SearchInput } from './form/search';
import { LoadingBar } from './loading';

export interface DataTableColumn<T> {
  label: string;
  get: (x: T) => JSX.Element;
  sortBy?: (x: T) => string;
}

export interface DataTableProps<T> {
  columns: DataTableColumn<T>[];
  items: T[];

  searchQuery?: string;
  setSearchQuery?: (query: string) => void;

  refresh?: () => void;
  isLoading?: boolean;
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

  const sortByColumn = (event: Event, column: DataTableColumn<T>) => {
    event.preventDefault();
    setSortComparator(() =>
      column.sortBy ? keyComparator(column.sortBy) : undefined,
    );
  };

  return (
    <div class='bg-gray-800 rounded-2 overflow-clip'>
      <div class='p-4 flex'>
        <Show when={props.searchQuery !== undefined}>
          <SearchInput
            query={props.searchQuery || ''}
            setQuery={props.setSearchQuery || (() => {})}
          />
        </Show>
        <div class='flex-grow' />
        <Show when={props.refresh}>
          <Button onclick={() => (props.refresh || (() => {}))()}>
            Refresh
          </Button>
        </Show>
      </div>
      <table class='font-sans border-collapse text-left w-full'>
        <thead class='text-xs uppercase text-gray-400 bg-gray-700'>
          <tr>
            <For each={props.columns}>
              {(column) => (
                <th class='px-4 py-3'>
                  <Show when={column.sortBy} fallback={column.label}>
                    <a
                      href=''
                      class='text-gray-400'
                      onClick={(event) => sortByColumn(event, column)}
                    >
                      {column.label}
                    </a>
                  </Show>
                </th>
              )}
            </For>
          </tr>
          <tr>
            <th class='line-height-0 m-0 p-0' colspan={props.columns.length}>
              <div classList={{ invisible: !props.isLoading }}>
                <LoadingBar />
              </div>
            </th>
          </tr>
        </thead>
        <tbody>
          <For each={sortedItems()}>
            {(item) => (
              <tr class='border-t border-t-style-solid border-gray-700'>
                <For each={props.columns}>
                  {(col) => (
                    <td class='px-4 py-3 font-medium text-white'>
                      {col.get(item)}
                    </td>
                  )}
                </For>
              </tr>
            )}
          </For>
        </tbody>
      </table>
    </div>
  );
}

const collator = new Intl.Collator();

function keyComparator<T>(key: (x: T) => string): (x: T, y: T) => number {
  return (x, y) => collator.compare(key(x), key(y));
}
