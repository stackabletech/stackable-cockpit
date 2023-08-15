import { For, createResource } from 'solid-js';

import { DisplayCondition, getStacklets } from '@/api/stacklets';
import { translate } from '@/localization';

import { DataTable } from '@/components/datatable';

import styles from './list.module.css';

export const Stacklets = () => {
  const [stacklets, { refetch }] = createResource(getStacklets);
  return (
    <div class='col-span-full'>
      <DataTable
        items={stacklets() || []}
        columns={[
          {
            label: translate('stacklet--product'),
            get: (x) => x.product,
            sortBy: (x) => x.product,
          },
          {
            label: translate('stacklet--namespace'),
            get: (x) => x.namespace || '(Cluster-scoped)',
            sortBy: (x) => x.namespace || '',
          },
          {
            label: translate('stacklet--name'),
            get: (x) => x.name,
            sortBy: (x) => x.name,
          },
          {
            label: translate('stacklet--status'),
            get: (x) => <StackletConditions conditions={x.conditions} />,
          },
          /* {
            label: 'Actions',
            get: (x) => (
              <ButtonLink href={`/stacklets/${x.namespace}/${x.name}/connect`}>
                Connect
              </ButtonLink>
            ),
          }, */
        ]}
        // extraButtons={
        //   <ButtonLink href='/stacklets/add' role='primary'>
        //     <AddSymbol />
        //     <span>Add stacklet</span>
        //   </ButtonLink>
        // }
        refresh={refetch}
        isLoading={stacklets.loading}
      />
    </div>
  );
};

const StackletConditions = (props: { conditions: DisplayCondition[] }) => (
  <ul class='p-0 m-0'>
    <For each={props.conditions}>
      {(cond) => (
        <li class={styles.inlineListItem}>
          <StackletCondition condition={cond} />
        </li>
      )}
    </For>
  </ul>
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
