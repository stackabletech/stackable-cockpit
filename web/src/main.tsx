import '@unocss/reset/sanitize/sanitize.css';
import 'virtual:uno.css';
import { render } from 'solid-js/web';
import { A, Route, Router, Routes } from '@solidjs/router';
import { Listeners } from './components/listeners';

const Home = () => {
    return <>
        lorem ipsum dolor sit amet
    </>;
}

const GlobalNav = () => {
    const NavItem = (props: any) =>
        <li class='inline mr-1'>
            <A href={props.href}
               class="p-1 c-white inline-block"
               activeClass='bg-stblue'
               inactiveClass='bg-stblue bg-opacity-50 hover:bg-opacity-80'>
                {props.children}
            </A>
        </li>;

    return <>
        <nav class='bg-gray'>
            <ul class='m-0 p-0'>
                <NavItem href="/listeners">listeners</NavItem>
                <NavItem href="/stacks">stacks</NavItem>
            </ul>
        </nav>
    </>
}

const App = () => {
    return <div class='w-5xl ma'>
        <h1>stackablectl, web edition</h1>
        <GlobalNav />
        <Routes>
            <Route path="/listeners" component={Listeners} />
            <Route path="/" component={Home} />
        </Routes>
    </div>
};

render(() => <Router base="/ui"><App/></Router>, document.getElementById("app")!!);
