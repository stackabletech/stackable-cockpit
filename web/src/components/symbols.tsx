import { FeatherIconNames, icons as featherIcons } from 'feather-icons';

interface FeatherSymbolProps {
  icon: FeatherIconNames;
}
const FeatherSymbol = (props: FeatherSymbolProps) => {
  const icon = () => featherIcons[props.icon];
  // Icon contents are provided statically by feather, and not influenced by user input
  // eslint-disable-next-line solid/no-innerhtml
  return <svg {...icon().attrs} innerHTML={icon().contents} />;
};

export const SearchSymbol = () => <FeatherSymbol icon='search' />;
export const AddSymbol = () => <FeatherSymbol icon='plus' />;
