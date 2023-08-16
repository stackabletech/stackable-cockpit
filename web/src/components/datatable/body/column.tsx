import { For, JSX, Show } from 'solid-js';

import styles from './column.module.scss';

export interface DataTableColumnSpec<T> {
  value: (r: T) => JSX.Element;
  sortBy?: (r: T) => string;
  name: string;
}

export interface DataTableColumnProps<T> extends DataTableColumnSpec<T> {
  items: T[];
}

export const DataTableColumn = <T,>(props: DataTableColumnProps<T>) => {
  return (
    <div class={styles.column}>
      <Show
        when={props.sortBy}
        fallback={<h4 class={styles.columnHeader}>{props.name}</h4>}
      >
        <h4
          class={styles.columnHeaderSortable}
          onClick={(event) => sortByColumn(event, props.sortBy)}
        >
          {props.name}
        </h4>
      </Show>
      <For each={props.items}>
        {(item) => <div class={styles.columnCell}>{props.value(item)}</div>}
      </For>
    </div>
  );
};

const sortByColumn = <T,>(event: Event, _sortBy?: (r: T) => string) => {
  event.preventDefault();
  console.log('Hey');
};
