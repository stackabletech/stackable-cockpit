import { Show, For, Switch, Match, createResource } from 'solid-js';
import { getListeners } from '../api';

// eslint-disable-next-line @typescript-eslint/naming-convention
export const Listeners = () => {
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
        <For each={listeners()}>{listener =>
          <tr>
            <td>{listener.product}</td>
            <td>{listener.metadata.namespace}</td>
            <td>{listener.metadata.name}</td>
            <td>
              <ul>
                <For each={listener.endpoints}>{endpoint =>
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
