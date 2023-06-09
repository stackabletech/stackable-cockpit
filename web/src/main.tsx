/* eslint-disable @typescript-eslint/naming-convention */
import '@unocss/reset/sanitize/sanitize.css';
import 'virtual:uno.css';
import { render } from 'solid-js/web';
import { A, Route, Router, Routes } from '@solidjs/router';
import type { ParentProps } from 'solid-js';
import { Listeners } from './pages/listeners';
import { ProductClusters } from './pages/product-clusters/list';
import { ProductClusterConnectionDetails } from './pages/product-clusters/connect';
import { Header } from './components/header';

const Home = () => {
  return <>lorem ipsum dolor sit amet</>;
};

const App = () => {
  return (
    <div class='bg-gray-900 min-h-screen c-white'>
      <Header />
      <div class='max-w-5xl mx-a mt-16'>
        <Routes>
          <Route
            path='/product-clusters/:namespace/:name/connect'
            component={ProductClusterConnectionDetails}
          />
          <Route path='/product-clusters' component={ProductClusters} />
          <Route path='/listeners' component={Listeners} />
          <Route path='/' component={Home} />
        </Routes>
      </div>
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
