interface MaterialSymbolProps {
  icon: string;
}
const MaterialSymbol = (props: MaterialSymbolProps) => (
  <span class='material-symbols-outlined vertical-middle'>{props.icon}</span>
);

export const SearchSymbol = () => <MaterialSymbol icon='search' />;
export const AddSymbol = () => <MaterialSymbol icon='add' />;
