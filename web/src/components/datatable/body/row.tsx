import { For, JSX, Show } from 'solid-js';

import styles from './row.module.scss';

export interface DataTableColumnSpec<T> {
  value: (r: T) => JSX.Element;
  sortBy?: (r: T) => string;
  name: string;
}

export interface DataTableHeaderRowProps<T> {
  columns: DataTableColumnSpec<T>[];
}

export const DataTableHeaderRow = <T,>(props: DataTableHeaderRowProps<T>) => {
  return (
    <thead>
      <tr>
        <For each={props.columns}>
          {(column) => (
            <Show
              when={column.sortBy}
              fallback={<th class={styles.headerRow}>{column.name}</th>}
            >
              <th
                class={styles.headerRowSortable}
                onClick={(event) => sortByColumn(event, column.sortBy)}
              >
                {column.name}
              </th>
            </Show>
          )}
        </For>
      </tr>
    </thead>
  );
};

const sortByColumn = <T,>(event: Event, _sortBy?: (r: T) => string) => {
  event.preventDefault();
  console.log('Hey');
};

export interface DataTableDataRowsProps<T> {
  columns: DataTableColumnSpec<T>[];
  items: T[];
}

export const DataTableDataRows = <T,>(props: DataTableDataRowsProps<T>) => {
  return (
    <tbody>
      <For each={props.items}>
        {(item) => <DataTableDataRow columns={props.columns} item={item} />}
      </For>
    </tbody>
  );
};

export interface DataTableDataRowProps<T> {
  columns: DataTableColumnSpec<T>[];
  item: T;
}

export const DataTableDataRow = <T,>(props: DataTableDataRowProps<T>) => {
  return (
    <tr>
      <For each={props.columns}>
        {(column) => <td class={styles.dataRow}>{column.value(props.item)}</td>}
      </For>
    </tr>
  );
};
