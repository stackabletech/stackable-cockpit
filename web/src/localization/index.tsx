import { FluentBundle, FluentResource, FluentVariable } from '@fluent/bundle';
import { mapBundleSync } from '@fluent/sequence';
import { negotiateLanguages } from '@fluent/langneg';
import { JSX, createContext, createMemo, useContext } from 'solid-js';
import { createLocalStorageSignal } from '../utils/localstorage';
import { Option } from '../types';

const fluentTranslationFiles = import.meta.glob('./locale/*.ftl', {
  eager: true,
  as: 'raw',
});
const translationBundles = Object.fromEntries(
  Object.entries(fluentTranslationFiles).map(([fileName, ftl]) => {
    const localeName = fileName.replace(
      /^.*\/(.+)\..*$/,
      (_, language: string) => language,
    );
    const resource = new FluentResource(ftl);
    const bundle = new FluentBundle(localeName);
    const errors = bundle.addResource(resource);
    if (errors.length > 0) {
      throw new Error(
        `Failed to load translation bundle ${fileName} as ${localeName}`,
        { cause: errors[0] },
      );
    }
    return [localeName, bundle];
  }),
);

const fallbackLocale = 'en';
interface LanguageContext {
  activeLocales(): string[];
  setUserLocale(language: Option<string>): void;
  availableLocales(): Record<string, FluentBundle>;
}
const LanguageContext = createContext<LanguageContext>();
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
  const [userLanguage, setUserLocale] =
    createLocalStorageSignal('user.language');
  const activeLocales = createMemo(() =>
    userLanguage().mapOrElse(
      () =>
        negotiateLanguages(
          navigator.languages,
          Object.keys(translationBundles),
          {
            defaultLocale: fallbackLocale,
          },
        ),
      (ul) => [ul, fallbackLocale],
    ),
  );
  return (
    <LanguageContext.Provider
      value={{
        activeLocales,
        setUserLocale,
        availableLocales: () => translationBundles,
      }}
    >
      {props.children}
    </LanguageContext.Provider>
  );
};

export const translate = (
  name: string,
  variables?: Record<string, FluentVariable>,
  options?: { overrideLocales?: string[] },
) => {
  const context = requireLanguageContext();
  const localeChain = options?.overrideLocales ?? context.activeLocales();
  const bundle = mapBundleSync(
    localeChain.map((locale) => translationBundles[locale]),
    name,
  );
  if (bundle == undefined) {
    console.error(
      `No translation found for ${name} (locale chain: ${localeChain.toString()})`,
    );
    return `#${name}#`;
  }
  const pattern = bundle.getMessage(name)?.value;
  if (pattern == undefined) {
    throw new Error(
      `Translation pattern for ${name} has no value (in bundle ${bundle.locales.toString()})`,
    );
  }
  return bundle.formatPattern(pattern, variables);
};
