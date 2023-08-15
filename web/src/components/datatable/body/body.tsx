import {
  DataTableColumn,
  DataTableColumnSpec,
} from '@/components/datatable/body/column';
import { For } from 'solid-js';

export interface DataTableBodyProps<T> {
  columns: DataTableColumnSpec<T>[];
  items: T[];
}

export const DataTableBody = <T,>(props: DataTableBodyProps<T>) => {
  return (
    <div class='grid grid-flow-col justify-stretch'>
      <For each={props.columns}>
        {(column) => <DataTableColumn {...column} items={props.items} />}
      </For>
    </div>
  );
};
