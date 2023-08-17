import { For, Show, createResource } from 'solid-js';
import { DisplayCondition, getStacklets } from '../../api/stacklets';
import { DataTable } from '../../components/datatable';
import styles from './list.module.css';
import { translate } from '../../localization';

export const Stacklets = () => {
  const [stacklets, { refetch }] = createResource(getStacklets);
  return (
    <>
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
            label: translate('stacklet--endpoints'),
            get: (x) => <StackletEndpoints endpoints={x.endpoints} />,
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
        extraButtons={
          <>
            {/* <ButtonLink href='/stacklets/add' role='primary'>
              <AddSymbol /> {translate('stacklet--add')}
            </ButtonLink> */}
          </>
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
