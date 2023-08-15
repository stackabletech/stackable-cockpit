import { FeatherIconNames } from 'feather-icons';
import { A } from '@solidjs/router';
import { Show } from 'solid-js';

import { FeatherSymbol } from '@/components/symbols';

import styles from './button.module.scss';

/// Special types of buttons that need specific callouts
export type ButtonRole = 'primary' | 'secondary';

export interface VisualButtonProps {
  icon?: FeatherIconNames;
  role?: ButtonRole;
  loading?: boolean;
  text: string;
}

const buttonProps = (props: VisualButtonProps) => {
  const roleStyles =
    // buttonProps is only called within a reactive scope
    // eslint-disable-next-line solid/reactivity
    props.role === 'primary' ? styles.btnPrimary : styles.btnSecondary;

  return {
    class: `${styles.btn} ${roleStyles}`,
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
    <Show
      when={props.icon}
      fallback={
        <Show when={props.loading}>
          <FeatherSymbol icon='loader' class='animate animate-spin' />
        </Show>
      }
    >
      {(icon) => (
        <Show
          when={!props.loading}
          fallback={<FeatherSymbol icon='loader' class='animate-spin' />}
        >
          <FeatherSymbol icon={icon()} />
        </Show>
      )}
    </Show>
    <span>{props.text}</span>
  </button>
);

export interface LinkButtonProps extends VisualButtonProps {
  href: string;
}

export const LinkButton = (props: LinkButtonProps) => (
  <A {...buttonProps(props)} href={props.href}>
    {props.children}
  </A>
);
