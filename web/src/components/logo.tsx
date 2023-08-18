import { A } from '@solidjs/router';

import logo from '@/resources/logo.svg';
import { Show } from 'solid-js';

export interface LogoProps {
  withLink?: boolean;
}

export const Logo = (props: LogoProps) => (
  <Show
    when={props.withLink}
    fallback={
      <img
        src={logo}
        elementtiming='logo'
        fetchpriority='auto'
        alt='Stackable Logo'
        class='h-20px'
      />
    }
  >
    <A href='/'>
      <img
        src={logo}
        elementtiming='logo'
        fetchpriority='auto'
        alt='Stackable Logo'
        class='h-20px'
      />
    </A>
  </Show>
);
