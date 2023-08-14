import { TextInput, TextInputProps } from '@/components/form/text-input';
import { SearchSymbol } from '@/components/symbols';

import styles from './search.module.scss';

interface SearchInputProps extends Omit<TextInputProps, 'placeholder'> {
  query: string;
  setQuery: (query: string) => void;
}

export const SearchInput = (props: SearchInputProps) => {
  return (
    <div class={styles.inputSearch}>
      <div class='icon'>
        <SearchSymbol />
      </div>
      <TextInput
        onInput={(event) => props.setQuery(event.target.value)}
        placeholder='Search...'
      />
    </div>
  );
};
