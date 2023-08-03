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
        class='inline flex-grow h-full b-none bg-transparent c-gray-200'
        placeholder='Search'
        value={props.query}
        onInput={(event) => props.setQuery(event.currentTarget.value)}
      />
    </div>
    // <label class='bg-gray-600 rounded flex flex-items-center c-gray-200'>
    //   <div class='p-1'>
    //     <SearchSymbol />
    //   </div>
    //   <input
    //     class='inline flex-grow h-full b-none bg-transparent c-gray-200'
    //     placeholder='Search'
    //     value={props.query}
    //     onInput={(event) => props.setQuery(event.currentTarget.value)}
    //   />
    // </label>
  );
};
