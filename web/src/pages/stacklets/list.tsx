import { For, Show, createResource } from 'solid-js';

import { DisplayCondition, getStacklets } from '@/api/stacklets';
import { translate } from '@/localization';

import { DataTable } from '@/components/datatable';

import styles from './list.module.css';

export const Stacklets = () => {
  // TODO (Techassi): Let's find a way to throttle spamming the refresh by making sure the request is done
  const [stacklets, { refetch }] = createResource(getStacklets);
  return (
    <div class='col-span-full mt-8'>
      <DataTable
        items={stacklets() || []}
        columns={[
          {
            name: translate('stacklet--product'),
            value: (x) => x.product,
            sortBy: (x) => x.product,
          },
          {
            name: translate('stacklet--namespace'),
            value: (x) => x.namespace || '(Cluster-scoped)',
            sortBy: (x) => x.namespace || '',
          },
          {
            name: translate('stacklet--name'),
            value: (x) => x.name,
            sortBy: (x) => x.name,
          },
          {
            name: translate('stacklet--endpoints'),
            value: (x) => <StackletEndpoints endpoints={x.endpoints} />,
          },
          {
            name: translate('stacklet--status'),
            value: (x) => <StackletConditions conditions={x.conditions} />,
          },
        ]}
        refresh={refetch}
        // isLoading={stacklets.loading}
      />
    </div>
  );
};

const StackletConditions = (props: { conditions: DisplayCondition[] }) => (
  <Show when={props.conditions.length > 0} fallback={<span>-</span>}>
    <ul class='p-0 m-0'>
      <For each={props.conditions}>
        {(cond) => (
          <li class={styles.inlineListItem}>
            <StackletCondition condition={cond} />
          </li>
        )}
      </For>
    </ul>
  </Show>
);

const StackletCondition = (props: { condition: DisplayCondition }) => (
  <span
    classList={{
      'c-green': props.condition.is_good === true,
      'c-red': props.condition.is_good === false,
    }}
    title={props.condition.message || undefined}
  >
    {props.condition.condition}
  </span>
);

const StackletEndpoints = (props: {
  endpoints: { [key: string]: string | undefined };
}) => (
  <ul class='p-0 m-0'>
    <For each={Object.entries(props.endpoints)}>
      {(item) => (
        <li class={styles.inlineListItem}>
          <Show
            when={
              item[1]?.startsWith('http://') || item[1]?.startsWith('https://')
            }
            fallback={
              <span class='c-white'>
                {item[0]}: {item[1]}
              </span>
            }
          >
            <a class='c-white' href={item[1]}>
              {item[0]}
            </a>
          </Show>
        </li>
      )}
    </For>
  </ul>
);
