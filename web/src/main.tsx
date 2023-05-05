import 'virtual:uno.css';
import { render } from 'solid-js/web';
import { Show, For, createResource, Switch, Match } from 'solid-js';
import { getListeners } from './api';
import { A, Route, Router, Routes } from '@solidjs/router';
import { Listeners } from './components/listeners';

const Home = () => {
    return <>
        lorem ipsum dolor sit amet
    </>;
}

const App = () => {
    return <>
        <h1>stackablectl, web edition</h1>
        <nav>
            <ul>
                <li><A href='/listeners'>listeners</A></li>
            </ul>
        </nav>
        <Routes>
            <Route path="/listeners" component={Listeners} />
            <Route path="/" component={Home} />
        </Routes>
    </>
};

render(() => <Router base="/ui"><App/></Router>, document.getElementById("app")!!);
