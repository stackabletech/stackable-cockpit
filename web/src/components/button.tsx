import { JSX } from 'solid-js';

export interface ButtonProps {
  children: JSX.Element;
  onclick: () => void;
}

export function Button(props: ButtonProps) {
  return (
    <button
      class='p-2 bg-gray-700 hover-bg-gray-600 active-bg-gray-500 c-white rounded border-1 border-solid border-gray-600 cursor-pointer'
      onclick={props.onclick}
    >
      {props.children}
    </button>
  );
}
