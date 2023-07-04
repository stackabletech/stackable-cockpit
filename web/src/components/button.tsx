import { A } from '@solidjs/router';
import { JSX } from 'solid-js';

/// Special types of buttons that need specific callouts
export type ButtonRole = 'primary';

export interface VisualButtonProps {
  children: JSX.Element;
  role?: ButtonRole;
}

const buttonProps = (props: VisualButtonProps) => {
  const roleClasses =
    // buttonProps is only called within a reactive scope
    // eslint-disable-next-line solid/reactivity
    props.role === 'primary'
      ? 'bg-stackable-blue-light hover-bg-stackable-blue-dark active-stackable-blue-dark border-stackable-blue-dark'
      : 'bg-gray-700 hover-bg-gray-600 active-bg-gray-500 border-gray-600';

  return {
    class: `p-2 text-size-4 c-white rounded border-1 border-solid  cursor-pointer decoration-none ${roleClasses}`,
  };
};

export interface ButtonProps extends VisualButtonProps {
  onClick: () => void;
}

export const Button = (props: ButtonProps) => (
  <button
    {...buttonProps(props)}
    onClick={(event) => {
      event.preventDefault();
      props.onClick();
    }}
  >
    {props.children}
  </button>
);

export interface ButtonLinkProps extends VisualButtonProps {
  href: string;
}

export const ButtonLink = (props: ButtonLinkProps) => (
  <A {...buttonProps(props)} href={props.href}>
    {props.children}
  </A>
);
