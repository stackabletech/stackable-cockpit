/* eslint-disable @typescript-eslint/naming-convention */
import '@unocss/reset/sanitize/sanitize.css';
import 'virtual:uno.css';

import { render } from 'solid-js/web';
import { Route, Router, Routes } from '@solidjs/router';

import { Stacklets, StackletConnectionDetails } from './pages/stacklets';
import { Wrapper } from './components/layout';
import { Listeners } from './pages/listeners';
import { Header } from './components/header';
import { LoginPageOr } from './pages/login';

import './scss/base.scss';

const Home = () => {
  return <>lorem ipsum dolor sit amet</>;
};

const App = () => {
  return (
    <Wrapper>
      <LoginPageOr>
        <Header />
        <div class='max-w-5xl mx-a mt-16'>
          <Routes>
            <Route
              path='/stacklets/:namespace/:name/connect'
              component={StackletConnectionDetails}
            />
            <Route path='/stacklets' component={Stacklets} />
            <Route path='/listeners' component={Listeners} />
            <Route path='/' component={Home} />
          </Routes>
        </div>
      </LoginPageOr>
    </Wrapper>
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
