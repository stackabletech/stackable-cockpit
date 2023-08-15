import { For, JSX } from 'solid-js';

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
    <div class='flex flex-col'>
      <h4 class='m-0 text-sm py-3 pl-4 text-gray-400 font-bold bg-gray-700'>
        {props.name}
      </h4>
      <For each={props.items}>{(item) => <div>{props.value(item)}</div>}</For>
    </div>
  );
};
