import { JSX } from 'solid-js';

import style from './wrapper.module.scss';

export interface WrapperProps {
  children: JSX.Element;
}

export const Wrapper = (props: WrapperProps) => (
  <main class={style.main}>{props.children}</main>
);
