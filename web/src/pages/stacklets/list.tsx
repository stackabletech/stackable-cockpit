import { For, createResource } from 'solid-js';
import { DisplayCondition, getStacklets } from '../../api/stacklets';
import { DataTable } from '../../components/datatable';
import { ButtonLink } from '../../components/button';
import { AddSymbol } from '../../components/symbols';

export const Stacklets = () => {
  const [stacklets, { refetch }] = createResource(getStacklets);
  return (
    <>
      <DataTable
        items={stacklets() || []}
        columns={[
          {
            label: 'Product',
            get: (x) => x.product,
            sortBy: (x) => x.product,
          },
          {
            label: 'Namespace',
            get: (x) => x.namespace || '(Cluster-scoped)',
            sortBy: (x) => x.namespace || '',
          },
          {
            label: 'Name',
            get: (x) => x.name,
            sortBy: (x) => x.name,
          },
          {
            label: 'Status',
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
        extraButtons={
          <ButtonLink href='/stacklets/add' role='primary'>
            <AddSymbol /> Add stacklet
          </ButtonLink>
        }
        refresh={refetch}
        isLoading={stacklets.loading}
      />
    </>
  );
};

const StackletConditions = (props: { conditions: DisplayCondition[] }) => (
  <ul class='p-0 m-0'>
    <For each={props.conditions}>
      {(cond) => (
        <li class='inline-list-item'>
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
