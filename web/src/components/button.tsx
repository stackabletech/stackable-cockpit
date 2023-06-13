import { A } from '@solidjs/router';
import { JSX } from 'solid-js';

/// Special types of buttons that need specific callouts
export type ButtonRole = 'primary';

export interface VisualButtonProps {
  children: JSX.Element;
  role?: ButtonRole;
}

const buttonProps = (props: VisualButtonProps) => {
  const roleClasses = () =>
    props.role === 'primary'
      ? 'bg-stbluelight hover-bg-stbluedark active-stbluedark border-stbluedark'
      : 'bg-gray-700 hover-bg-gray-600 active-bg-gray-500 border-gray-600';

  return {
    class: `p-2 text-size-4 c-white rounded border-1 border-solid  cursor-pointer decoration-none ${roleClasses()}`,
  };
};

export interface ButtonProps extends VisualButtonProps {
  onclick: () => void;
}

export const Button = (props: ButtonProps) => (
  <button {...buttonProps(props)} onclick={props.onclick}>
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
