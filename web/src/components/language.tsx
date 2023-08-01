import { For } from 'solid-js';
import { requireLanguageContext, translate } from '../localization';
import { Some } from '../types/option';

export const LanguagePicker = () => {
  const [activeLanguages, { setUserLanguage, availableLanguages }] =
    requireLanguageContext();
  return (
    <select onInput={(event) => setUserLanguage(Some(event.target.value))}>
      <For each={Object.keys(availableLanguages())}>
        {(language) => (
          <option value={language} selected={activeLanguages()[0] === language}>
            {translate('language-name', {}, { overrideLanguages: [language] })}
          </option>
        )}
      </For>
    </select>
  );
};
