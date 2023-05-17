import { Show, For, Switch, Match, createResource } from 'solid-js';
import { getListeners } from '../api';
import { DataTable } from './datatable';

export const Listeners = () => {
  const [listeners, { refetch }] = createResource(getListeners);
  return (
    <>
      <button onClick={refetch}>Refresh</button>
      <Show when={listeners.loading}>Loading...</Show>
      <DataTable
        items={listeners() || []}
        columns={[
          {
            label: 'Product',
            get: (listener) => listener.product,
            sortable: true,
          },
          {
            label: 'Namespace',
            get: (listener) => listener.metadata.namespace,
            sortable: true,
          },
          {
            label: 'Name',
            get: (listener) => listener.metadata.name,
            sortable: true,
          },
          {
            label: 'Endpoints',
            get: (listener) => (
              <ul>
                <For each={listener.endpoints}>
                  {(endpoint) => (
                    <li>
                      <Switch fallback={endpoint.path}>
                        <Match when={endpoint.web}>
                          <a href={endpoint.path}>{endpoint.path}</a>
                        </Match>
                      </Switch>
                    </li>
                  )}
                </For>
              </ul>
            ),
          },
          { label: 'Info', get: (_) => '' },
        ]}
      />
    </>
  );
};
