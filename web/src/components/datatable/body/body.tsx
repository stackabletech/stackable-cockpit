import {
  DataTableHeaderRow,
  DataTableColumnSpec,
  DataTableDataRows,
} from '@/components/datatable';

import styles from './body.module.scss';

export interface DataTableBodyProps<T> {
  columns: DataTableColumnSpec<T>[];
  items: T[];
}

export const DataTableBody = <T,>(props: DataTableBodyProps<T>) => {
  return (
    <table class={styles.dataTable}>
      <DataTableHeaderRow columns={props.columns} />
      <DataTableDataRows {...props} />
    </table>
  );
};
