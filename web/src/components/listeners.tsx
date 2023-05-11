import { Show, For, Switch, Match, createResource, JSX } from "solid-js";
import { getListeners } from "../api";

interface DataTableColumn<T> {
    label: string,
    get: (x: T) => JSX.Element,
}
interface DataTableProps<T> {
    columns: DataTableColumn<T>[],
    items: T[],
}
function DataTable<T>(props: DataTableProps<T>): JSX.Element {
    return <>
        <table>
            <thead>
                <tr>
                    <For each={props.columns}>{(col) =>
                        <th>
                            {col.label}
                        </th>
                    }</For>
                </tr>
            </thead>
            <tbody>
                <For each={props.items}>{(item) =>
                    <tr>
                        <For each={props.columns}>{(col) =>
                            <td>{col.get(item)}</td>
                        }</For>
                    </tr>
                }</For>
            </tbody>
        </table>
    </>;
}

export const Listeners = () => {
    const [listeners, { refetch }] = createResource(getListeners);
    return <>
        <button onClick={refetch}>Refresh</button>
        <Show when={listeners.loading}>Loading...</Show>
        <DataTable items={listeners() || []} columns={[
            { label: "Product", get: listener => listener.product },
            { label: "Namespace", get: listener => listener.metadata.namespace },
            { label: "Name", get: listener => listener.metadata.name },
            {
                label: "Endpoints",
                get: listener => <ul>
                    <For each={listener.endpoints}>{(endpoint) =>
                        <li>
                            <Switch fallback={endpoint.path}>
                                <Match when={endpoint.web}><a href={endpoint.path}>{endpoint.path}</a></Match>
                            </Switch>
                        </li>
                    }</For>
                </ul>
            },
            { label: "Info", get: _ => "" }
        ]} />
    </>;
};
