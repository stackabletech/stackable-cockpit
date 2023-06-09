import {
  Show,
  For,
  Switch,
  Match,
  createResource,
  createSignal,
  createMemo,
} from 'solid-js';
import { getListeners } from '../api';
import { DataTable } from '../components/datatable';

export const Listeners = () => {
  const [listeners, { refetch }] = createResource(getListeners);
  const [searchQuery, setSearchQuery] = createSignal('');
  const filteredListeners = createMemo(() => {
    const query = searchQuery();
    // TODO: Placeholder search logic
    return listeners()?.filter((x) => x.metadata.name.includes(query));
  });
  return (
    <>
      <DataTable
        items={filteredListeners() || []}
        searchQuery={searchQuery()}
        setSearchQuery={setSearchQuery}
        isLoading={listeners.loading}
        columns={[
          {
            label: 'Product',
            get: (listener) => listener.product,
            sortBy: (listener) => listener.product,
          },
          {
            label: 'Namespace',
            get: (listener) => listener.metadata.namespace,
            sortBy: (listener) => listener.metadata.namespace,
          },
          {
            label: 'Name',
            get: (listener) => listener.metadata.name,
            sortBy: (listener) => listener.metadata.name,
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
          { label: 'Info', get: () => '' },
        ]}
        refresh={refetch}
      />
    </>
  );
};
