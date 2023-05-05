import 'virtual:uno.css';
import { render } from 'solid-js/web';
import { Show, For, createResource, Switch, Match } from 'solid-js';
import { getListeners } from './api';
import { A, Route, Router, Routes } from '@solidjs/router';

const Listeners = () => {
    const [listeners, { refetch }] = createResource(getListeners);
    return <>
        <button onClick={refetch}>Refresh</button>
        <Show when={listeners.loading}>Loading...</Show>
        <table>
            <thead>
                <tr>
                    <th>Product</th>
                    <th>Namespace</th>
                    <th>Name</th>
                    <th>Endpoints</th>
                    <th>Info</th>
                </tr>
            </thead>
            <tbody>
                <For each={listeners()}>{(listener) =>
                    <tr>
                        <td>{listener.product}</td>
                        <td>{listener.metadata.namespace}</td>
                        <td>{listener.metadata.name}</td>
                        <td>
                            <ul>
                                <For each={listener.endpoints}>{(endpoint) =>
                                    <li>
                                        <Switch fallback={endpoint.path}>
                                            <Match when={endpoint.web}><a href={endpoint.path}>{endpoint.path}</a></Match>
                                        </Switch>
                                    </li>
                                }</For>
                            </ul>
                        </td>
                    </tr>
                }</For>
            </tbody>
        </table>
    </>;
};

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
