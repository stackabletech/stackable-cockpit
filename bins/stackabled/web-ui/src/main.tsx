import 'virtual:uno.css';
import { render } from 'solid-js/web';
import { createSignal } from 'solid-js';

function MyComponent() {
    const [counter, setCounter] = createSignal(0);
    const increment = () => {setCounter(c => c + 1)};
    return <div class="c-yellow">hi <button onclick={increment}>{counter()}</button></div>;
}

render(MyComponent, document.getElementById("app")!!);
