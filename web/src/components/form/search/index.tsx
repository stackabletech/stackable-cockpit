import { SearchSymbol } from '@/components/symbols';

import styles from './search.module.scss';

interface SearchInputProps {
  query: string;
  setQuery: (query: string) => void;
}

export const SearchInput = (props: SearchInputProps) => {
  return (
    <div class={styles.inputSearch}>
      <div class='icon'>
        <SearchSymbol />
      </div>
      <input
        placeholder='Search'
        value={props.query}
        onInput={(event) => props.setQuery(event.currentTarget.value)}
      />
    </div>
  );
};
