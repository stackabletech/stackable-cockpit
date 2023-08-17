import { A } from '@solidjs/router';

import { translate } from '@/localization';
import { logOut } from '@/api/session';

import { LanguagePicker } from '@/components/language';
import { Button } from '@/components/button';
import { Logo } from '@/components/logo';

interface NavItemProps {
  href: string;
  text: string;
}

const NavItem = (props: NavItemProps) => {
  return (
    <li class='block h-auto ml-4'>
      <A href={props.href || ''}>{props.text}</A>
    </li>
  );
};

export const Header = () => {
  return (
    <header class='bg-gray-800 px-6 py-2'>
      <nav class='flex'>
        <Logo withLink={true} />
        <ul class='flex-auto m-0 p-0 flex'>
          <NavItem href='/stacklets' text={translate('stacklet--list')} />
          <li class='flex-grow' />
          <LanguagePicker />
          <Button
            text={translate('login--log-out')}
            role='secondary'
            onClick={logOut}
          />
        </ul>
      </nav>
    </header>
  );
};
