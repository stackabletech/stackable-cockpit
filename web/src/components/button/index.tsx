import { A } from '@solidjs/router';
import { JSX } from 'solid-js';

import styles from './button.module.scss';

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
    props.role === 'primary' ? styles.btnPrimary : styles.btnSecondary;

  return {
    class: `${styles.btn} ${roleClasses}`,
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
