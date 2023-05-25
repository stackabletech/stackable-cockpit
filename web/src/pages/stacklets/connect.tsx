import {
  For,
  Match,
  Show,
  Switch,
  createResource,
  createUniqueId,
} from 'solid-js';
import { DiscoveryFieldType, getStackletDiscovery } from '../../api/index';
import { Params, useParams } from '@solidjs/router';

interface StackletConnectionDetailsParams extends Params {
  namespace: string;
  name: string;
}

export const StackletConnectionDetails = () => {
  const params = useParams<StackletConnectionDetailsParams>();
  const [discoveryConfig, { refetch }] = createResource(() =>
    getStackletDiscovery(params.namespace, params.name),
  );
  const configParams = () => {
    const currentDiscoveryConfig = discoveryConfig();
    const data = currentDiscoveryConfig?.data || {};
    const types = currentDiscoveryConfig?.fieldTypes || {};
    return Object.keys(data)
      .sort()
      .map((key) => ({ key, value: data[key], type: types[key] || 'blob' }));
  };
  return (
    <>
      <button onClick={refetch}>Refresh</button>
      <Show when={discoveryConfig.loading}>Loading...</Show>
      <ul>
        <For each={configParams()}>
          {(item) => (
            <li>
              <Field label={item.key} value={item.value} type={item.type} />
            </li>
          )}
        </For>
      </ul>
    </>
  );
};

interface FieldProps {
  label: string;
  value: string;
  type: DiscoveryFieldType;
}

const Field = (props: FieldProps) => {
  const dataFieldId = createUniqueId();
  return (
    <Switch>
      <Match when={props.type == 'url'}>
        <a href={props.value}>{props.label}</a>
      </Match>
      {/* Fall back to rendering as blob if unknown */}
      <Match when={props.type == 'blob' || true}>
        <label class='block' for={dataFieldId}>
          {props.label}
        </label>
        <textarea class='block' id={dataFieldId} readonly>
          {props.value}
        </textarea>
      </Match>
    </Switch>
  );
};
