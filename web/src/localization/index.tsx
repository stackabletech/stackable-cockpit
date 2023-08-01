import { FluentBundle, FluentResource } from '@fluent/bundle';
import { mapBundleSync } from '@fluent/sequence';
import { negotiateLanguages } from '@fluent/langneg';
import { Accessor, JSX, createContext, createMemo, useContext } from 'solid-js';
import { createLocalStorageSignal } from '../utils/localstorage';
import { Option } from '../types';

const ftls = import.meta.glob('./locale/*.ftl', { eager: true, as: 'raw' });
const bundles = Object.fromEntries(
  Object.entries(ftls).map(([fileName, ftl]) => {
    const languageName = fileName.replace(
      /^.*\/(.+)\..*$/,
      (_, language: string) => language,
    );
    const resource = new FluentResource(ftl);
    const bundle = new FluentBundle(languageName);
    const errors = bundle.addResource(resource);
    if (errors.length > 0) {
      throw new Error(
        `Failed to load translation bundle ${fileName} as ${languageName}`,
        { cause: errors[0] },
      );
    }
    return [languageName, bundle];
  }),
);

const fallbackLanguage = 'en';
const LanguageContext = createContext<
  [
    Accessor<string[]>,
    {
      setUserLanguage: (language: Option<string>) => void;
      availableLanguages: () => { [language: string]: FluentBundle };
    },
  ]
>();
export const requireLanguageContext = () => {
  const langContext = useContext(LanguageContext);
  if (langContext == undefined) {
    throw new Error(
      'LanguageContext is required but no LanguageProvider is currently active',
    );
  }
  return langContext;
};

export const LanguageProvider = (props: { children: JSX.Element }) => {
  const [userLanguage, setUserLanguage] =
    createLocalStorageSignal('user.language');
  const activeLanguages = createMemo(() =>
    userLanguage().mapOrElse(
      () =>
        negotiateLanguages(navigator.languages, Object.keys(bundles), {
          defaultLocale: fallbackLanguage,
        }),
      (ul) => [ul, fallbackLanguage],
    ),
  );
  return (
    <LanguageContext.Provider
      value={[
        activeLanguages,
        { setUserLanguage, availableLanguages: () => bundles },
      ]}
    >
      {props.children}
    </LanguageContext.Provider>
  );
};

export const translate = (
  name: string,
  variables?: { [variable: string]: string | number },
  options?: { overrideLanguages?: string[] },
) => {
  const [languages] = requireLanguageContext();
  const languageChain = options?.overrideLanguages ?? languages();
  const bundle = mapBundleSync(
    languageChain.map((language) => bundles[language]),
    name,
  );
  if (bundle == undefined) {
    console.error(
      `No translation found for ${name} (language chain: ${languageChain.toString()})`,
    );
    return `#${name}#`;
  }
  const pattern = bundle.getMessage(name)?.value;
  if (pattern == undefined) {
    throw new Error('Translation pattern for ${name} has no value');
  }
  return bundle.formatPattern(pattern, variables);
};
