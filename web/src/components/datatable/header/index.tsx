import { JSX, Show } from 'solid-js';

import styles from './header.module.scss';
import { SearchInput } from '@/components/form/search';

export interface DataTableHeaderProps {
  searchbar?: DataTableHeaderSearchbarProps;
  buttons?: JSX.Element;
}

export interface DataTableHeaderSearchbarProps {
  enabled?: boolean;
  onQuery?: (query: string) => void;
}

export const DataTableHeader = (props: DataTableHeaderProps) => {
  return (
    <div class={styles.tableHeader}>
      <div>
        <Show when={props.searchbar && props.searchbar.enabled}>
          <SearchInput onQuery={(query) => props.searchbar?.onQuery?.(query)} />
        </Show>
      </div>
      <div class={styles.tableHeaderButtons}>
        <Show when={props.buttons}>{(buttons) => buttons()}</Show>
      </div>
    </div>
  );
};
