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
    <li>
      <A href={props.href} class='text-gray-300 no-underline text-14px'>
        {props.text}
      </A>
    </li>
  );
};

export const Header = () => {
  return (
    <header class='bg-gray-800 px-6 py-2'>
      <nav class='flex justify-between items-center'>
        <div class='flex'>
          <Logo withLink={true} />
          <ul class='ml-9 flex m-0 gap-3'>
            <NavItem href='/stacklets' text={translate('stacklet--list')} />
          </ul>
        </div>
        <div class='flex gap-3'>
          <LanguagePicker />
          <Button
            text={translate('login--log-out')}
            role='secondary'
            onClick={logOut}
          />
        </div>
      </nav>
    </header>
  );
};
