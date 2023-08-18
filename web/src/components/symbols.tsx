import { FeatherIconNames, icons as featherIcons } from 'feather-icons';
import { JSX, splitProps } from 'solid-js';

interface FeatherSymbolProps extends JSX.SvgSVGAttributes<SVGSVGElement> {
  icon: FeatherIconNames;
}

export const FeatherSymbol = (props: FeatherSymbolProps) => {
  const [, rest] = splitProps(props, ['icon']);
  const icon = () => featherIcons[props.icon];
  // Icon contents are provided statically by feather, and not influenced by user input
  // eslint-disable-next-line solid/no-innerhtml
  return <svg {...icon().attrs} {...rest} innerHTML={icon().contents} />;
};

export const SearchSymbol = () => <FeatherSymbol icon='search' />;
export const AddSymbol = () => <FeatherSymbol icon='plus' />;
