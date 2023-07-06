/* eslint-disable @typescript-eslint/naming-convention */
import '@unocss/reset/sanitize/sanitize.css';
import 'virtual:uno.css';
import './main.css';
import { render } from 'solid-js/web';
import { Route, Router, Routes } from '@solidjs/router';
import { Listeners } from './pages/listeners';
import { Header } from './components/header';
import { StackletConnectionDetails } from './pages/stacklets/connect';
import { Stacklets } from './pages/stacklets/list';
import { LoginPageOr } from './pages/login';

const Home = () => {
  return <>lorem ipsum dolor sit amet</>;
};

const App = () => {
  return (
    <div class='bg-gray-900 min-h-screen c-white'>
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
