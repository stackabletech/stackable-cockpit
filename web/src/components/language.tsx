import { For } from 'solid-js';
import { requireLanguageContext, translate } from '../localization';
import { Some } from '../types/option';

export const LanguagePicker = () => {
  const context = requireLanguageContext();
  return (
    <select
      onInput={(event) => context.setUserLocale(Some(event.target.value))}
    >
      <For each={Object.keys(context.availableLocales())}>
        {(locale) => (
          <option
            value={locale}
            selected={context.activeLocales()[0] === locale}
          >
            {translate('locale-name', {}, { overrideLocales: [locale] })}
          </option>
        )}
      </For>
    </select>
  );
};
