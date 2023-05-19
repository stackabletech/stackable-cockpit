import { For, Show, createResource, createUniqueId } from 'solid-js';
import { getProductClusterDiscovery } from '../../api';
import { Params, useParams } from '@solidjs/router';

interface ProductClusterConnectionDetailsProps extends Params {
  namespace: string;
  name: string;
}

export const ProductClusterConnectionDetails = () => {
  const params = useParams<ProductClusterConnectionDetailsProps>();
  const [discoveryConfig, { refetch }] = createResource(() =>
    getProductClusterDiscovery(params.namespace, params.name),
  );
  const configParams = () => {
    const data = discoveryConfig()?.data || {};
    return Object.keys(data)
      .sort()
      .map((key) => ({ key, value: data[key] }));
  };
  return (
    <>
      <button onClick={refetch}>Refresh</button>
      <Show when={discoveryConfig.loading}>Loading...</Show>
      <ul>
        <For each={configParams()}>
          {(item) => {
            const textareaId = createUniqueId();
            return (
              <li>
                <label class='block' for={textareaId}>
                  {item.key}
                </label>
                <textarea class='block' id={textareaId} readonly>
                  {item.value}
                </textarea>
              </li>
            );
          }}
        </For>
      </ul>
    </>
  );
};
