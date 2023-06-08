import { A } from '@solidjs/router';
import { ParentProps } from 'solid-js';
import logo from '../resources/logo.png';

interface NavItemProps {
  href: string;
}

const NavItem = (props: ParentProps<NavItemProps>) => (
  <li class='block h-auto ml-4'>
    <A
      href={props.href}
      class='p-4 c-white flex flex-items-center h-full decoration-none bg-gray-900'
      inactiveClass='bg-opacity-30 hover:bg-opacity-50'
    >
      {props.children}
    </A>
  </li>
);

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
        <NavItem href='/product-clusters'>Product Clusters</NavItem>
        <NavItem href='/listeners'>Listeners</NavItem>
        <NavItem href='/stacks'>Stacks</NavItem>
      </ul>
    </nav>
  );
};
