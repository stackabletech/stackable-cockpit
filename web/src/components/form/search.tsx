interface SearchInputProps {
  query: string;
  setQuery: (query: string) => void;
}
export const SearchInput = (props: SearchInputProps) => {
  return (
    <label class='bg-gray-600 rounded flex flex-items-center c-gray-200'>
      <span class='p-1 material-symbols-outlined'>search</span>
      <input
        class='inline flex-grow h-full b-none bg-transparent c-gray-200'
        placeholder='Search'
        value={props.query}
        oninput={(event) => props.setQuery(event.currentTarget.value)}
      />
    </label>
  );
};
