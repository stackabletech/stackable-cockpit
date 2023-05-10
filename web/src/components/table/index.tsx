/* eslint-disable @typescript-eslint/naming-convention */
import { For, Show, mergeProps } from 'solid-js';
import type { Component } from 'solid-js';

export interface TableProps {
  columns?: string[];
  rows?: string[][];
}

const Table: Component<TableProps> = (props) => {
  props = mergeProps({ columns: [], rows: [] }, props);

  return (
    <table class="font-sans border-collapse text-left w-full">
      <Show when={props.columns && props.columns.length > 0}>
        <thead class="text-xs uppercase text-gray-400 bg-gray-700">
          <For each={props.columns}>{column =>
            <th class="px-4 py-3">{column}</th>
          }</For>
        </thead>
      </Show>
      <Show when={props.rows && props.rows.length > 0}>
        <tbody>
          <For each={props.rows}>{row =>
            <tr class="bg-gray-800 border-b border-b-style-solid border-gray-700">
              <For each={row}>{(cell, index) =>
                <td class="px-4 py-3 font-medium text-gray-400" classList={{ 'text-white': index() === 0 }}>{cell}</td>
              }</For>
            </tr>
          }</For>
        </tbody>
      </Show>
    </table >
  );
};

export default Table;