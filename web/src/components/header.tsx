import { A } from '@solidjs/router';
import { ParentProps, Show } from 'solid-js';
import logo from '../resources/logo.png';
import { logOut } from '../api/session';

interface NavItemProps {
  href?: string;
  onClick?: () => void;
}

const NavItem = (props: ParentProps<NavItemProps>) => {
  const linkClass =
    'p-4 b-0 cursor-pointer c-white flex flex-items-center h-full decoration-none bg-gray-900';
  const inactiveClass = 'bg-opacity-30 hover:bg-opacity-50';

  return (
    <li class='block h-auto ml-4'>
      <Show
        when={props.href !== undefined}
        fallback={
          <button
            class={`${linkClass} ${inactiveClass}`}
            onClick={(event) => {
              event.preventDefault();
              props.onClick?.();
            }}
          >
            {props.children}
          </button>
        }
      >
        <A
          href={props.href || ''}
          class={linkClass}
          inactiveClass={inactiveClass}
          onClick={() => props.onClick?.()}
        >
          {props.children}
        </A>
      </Show>
    </li>
  );
};

export const Header = () => {
  return (
    <nav class='flex bg-gray-600 h-16 px-4'>
      <h1 class='m-0 c-white'>
        <A class='flex flex-items-center h-full' href='/'>
          <img
            src={logo}
            elementtiming='logo'
            fetchpriority='auto'
            alt='Stackable'
          />
        </A>
      </h1>
      <ul class='flex-auto m-0 p-0 flex'>
        <NavItem href='/stacklets'>Stacklets</NavItem>
        <li class='flex-grow' />
        <NavItem onClick={() => logOut()}>Log out</NavItem>
      </ul>
    </nav>
  );
};
