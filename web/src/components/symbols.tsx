import { FeatherIconNames, icons as featherIcons } from 'feather-icons';

interface FeatherSymbolProps {
  icon: FeatherIconNames;
}
const FeatherSymbol = (props: FeatherSymbolProps) => {
  const icon = featherIcons[props.icon];
  return <svg {...icon.attrs} innerHTML={icon.contents} />;
};

export const SearchSymbol = () => <FeatherSymbol icon='search' />;
export const AddSymbol = () => <FeatherSymbol icon='plus' />;
