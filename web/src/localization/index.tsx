import { FluentBundle, FluentResource } from '@fluent/bundle';
import { mapBundleSync } from '@fluent/sequence';
import { Accessor, JSX, createContext, useContext } from 'solid-js';
import { createLocalStorageSignal } from '../utils/localstorage';
import { Option } from '../types';

const ftls = import.meta.glob('./locale/*.ftl', { eager: true, as: 'raw' });
const bundles = Object.fromEntries(
  Object.entries(ftls).map(([fileName, ftl]) => {
    const languageName = fileName.replace(
      /^.*\/(.+)\..*$/,
      (_, language) => language,
    );
    const resource = new FluentResource(ftl);
    const bundle = new FluentBundle(languageName);
    const errors = bundle.addResource(resource);
    if (errors.length) {
      throw new Error(
        `Failed to load translation bundle ${fileName} as ${languageName}: ${errors}`,
      );
    }
    return [languageName, bundle];
  }),
);

const fallbackLanguages = ['en'];
export const LanguageContext = createContext<
  [
    Accessor<string[]>,
    {
      setUserLanguage: (language: Option<string>) => void;
      availableLanguages: () => { [language: string]: FluentBundle };
    },
  ]
>();

export const LanguageProvider = (props: { children: JSX.Element }) => {
  const [userLanguage, setUserLanguage] =
    createLocalStorageSignal('user.language');
  const activeLanguages = () =>
    userLanguage().mapOr(fallbackLanguages, (ul) => [ul, ...fallbackLanguages]);
  const availableLanguages = () => bundles;
  return (
    <LanguageContext.Provider
      value={[activeLanguages, { setUserLanguage, availableLanguages }]}
    >
      {props.children}
    </LanguageContext.Provider>
  );
};

export const translate = (
  name: string,
  variables?: { [variable: string]: any },
  opts?: { overrideLanguages?: string[] },
) => {
  const [languages] = useContext(LanguageContext)!;
  const languageChain = opts?.overrideLanguages ?? languages();
  const bundle = mapBundleSync(
    languageChain.map((language) => bundles[language]),
    name,
  );
  if (bundle == null) {
    throw new Error(
      `No translation found for ${name} (language chain: ${languageChain})`,
    );
  }
  return bundle.formatPattern(bundle.getMessage(name)!.value!, variables);
};
