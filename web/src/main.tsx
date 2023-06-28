/* eslint-disable @typescript-eslint/naming-convention */
import '@unocss/reset/sanitize/sanitize.css';
import 'virtual:uno.css';
import { render } from 'solid-js/web';
import { A, Route, Router, Routes } from '@solidjs/router';
import type { ParentProps } from 'solid-js';
import { Listeners } from './pages/listeners';
import { Stacklets } from './pages/stacklets/list';
import { StackletConnectionDetails } from './pages/stacklets/connect';
import { LoginPageOr } from './pages/login';
import { logOut } from './api';

const Home = () => {
  return <>lorem ipsum dolor sit amet</>;
};

interface NavItemProps {
  href: string;
}

const GlobalNav = () => {
  const NavItem = (props: ParentProps<NavItemProps>) => (
    <li class='inline mr-1'>
      <A
        href={props.href}
        class='p-1 c-white inline-block'
        activeClass='bg-stblue'
        inactiveClass='bg-stblue bg-opacity-50 hover:bg-opacity-80'
      >
        {props.children}
      </A>
    </li>
  );

  return (
    <>
      <nav class='bg-gray'>
        <ul class='m-0 p-0'>
          <NavItem href='/stacklets'>stacklets</NavItem>
          <NavItem href='/listeners'>listeners</NavItem>
          <NavItem href='/stacks'>stacks</NavItem>
          <button onClick={() => logOut()}>Log out</button>
        </ul>
      </nav>
    </>
  );
};

const App = () => {
  return (
    <div class='max-w-5xl ma'>
      <h1>stackablectl, web edition</h1>
      <LoginPageOr>
        <GlobalNav />
        <Routes>
          <Route
            path='/stacklets/:namespace/:name/connect'
            component={StackletConnectionDetails}
          />
          <Route path='/stacklets' component={Stacklets} />
          <Route path='/listeners' component={Listeners} />
          <Route path='/' component={Home} />
        </Routes>
      </LoginPageOr>
    </div>
  );
};

const root = document.querySelector('#app');
if (root == undefined) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got mispelled?',
  );
} else {
  render(
    () => (
      <Router base='/ui'>
        <App />
      </Router>
    ),
    root,
  );
}
