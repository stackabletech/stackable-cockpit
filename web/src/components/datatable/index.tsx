import { splitProps } from 'solid-js';

import { DataTableBody, DataTableColumnSpec } from './body';
import { DataTableHeader, DataTableHeaderProps } from './header';
import { DataTableFooter } from './footer';

// export interface DataTableColumn<T> {
//   label: string;
//   get: (x: T) => JSX.Element;
//   sortBy?: (x: T) => string;
// }

// export interface DataTableProps<T> {
//   columns: DataTableColumn<T>[];
//   items: T[];

//   searchQuery?: string;
//   setSearchQuery?: (query: string) => void;

//   extraButtons?: JSX.Element;

//   refresh?: () => void;
//   isLoading?: boolean;
// }

// export function DataTable<T>(props: DataTableProps<T>): JSX.Element {
//   const [sortComparator, setSortComparator] = createSignal<
//     ((x: T, y: T) => number) | undefined
//   >();

//   const sortedItems = createMemo(() => {
//     const currentSortComparator = sortComparator();
//     if (currentSortComparator == undefined) {
//       return props.items;
//     } else {
//       const items = [...props.items];
//       items.sort(currentSortComparator);
//       return items;
//     }
//   });

//   const sortByColumn = (event: Event, column: DataTableColumn<T>) => {
//     event.preventDefault();
//     setSortComparator(() =>
//       column.sortBy ? keyComparator(column.sortBy) : undefined,
//     );
//   };

//   return (
//     <>
//       <DataTableHeader
//         buttons={
//           <>
//             {props.extraButtons}
//             <Button onClick={() => props.refresh?.()} text='Refresh' />
//           </>
//         }
//         searchbar={{ enabled: true, onQuery: (q) => props.setSearchQuery?.(q) }}
//       />
//       <table class='font-sans border-collapse text-left w-full'>
//         <thead class='text-xs uppercase text-gray-400 bg-gray-700'>
//           <tr>
//             <For each={props.columns}>
//               {(column) => (
//                 <th class='px-4 py-3'>
//                   <Show when={column.sortBy} fallback={column.label}>
//                     <a
//                       href=''
//                       class='text-gray-400'
//                       onClick={(event) => sortByColumn(event, column)}
//                     >
//                       {column.label}
//                     </a>
//                   </Show>
//                 </th>
//               )}
//             </For>
//           </tr>
//           <tr>
//             <th class='line-height-0 m-0 p-0' colspan={props.columns.length}>
//               <div classList={{ invisible: !props.isLoading }}>
//                 <LoadingBar />
//               </div>
//             </th>
//           </tr>
//         </thead>
//         <tbody>
//           <For each={sortedItems()}>
//             {(item) => (
//               <tr class='border-t border-t-style-solid border-gray-700'>
//                 <For each={props.columns}>
//                   {(col) => (
//                     <td class='px-4 py-3 font-medium text-white'>
//                       {col.get(item)}
//                     </td>
//                   )}
//                 </For>
//               </tr>
//             )}
//           </For>
//         </tbody>
//       </table>
//     </>
//   );
// }

// const collator = new Intl.Collator();

// function keyComparator<T>(key: (x: T) => string): (x: T, y: T) => number {
//   return (x, y) => collator.compare(key(x), key(y));
// }

export interface DataTableProps<T> extends DataTableHeaderProps {
  columns: DataTableColumnSpec<T>[];
  refresh: () => void;
  items: T[];
}

export const DataTable = <T,>(props: DataTableProps<T>) => {
  const [header, rest] = splitProps(props, ['searchbar', 'buttons', 'refresh']);

  return (
    <>
      <DataTableHeader {...header} />
      <DataTableBody {...rest} />
      <DataTableFooter />
    </>
  );
};

export * from './footer';
export * from './header';
export * from './body';
