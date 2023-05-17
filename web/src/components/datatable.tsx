import { For, JSX } from 'solid-js';

export interface DataTableColumn<T> {
  label: string;
  get: (x: T) => JSX.Element;
}
export interface DataTableProps<T> {
  columns: DataTableColumn<T>[];
  items: T[];
}
export function DataTable<T>(props: DataTableProps<T>): JSX.Element {
  return (
    <>
      <table>
        <thead>
          <tr>
            <For each={props.columns}>{(col) => <th>{col.label}</th>}</For>
          </tr>
        </thead>
        <tbody>
          <For each={props.items}>
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
