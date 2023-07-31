import { FluentBundle, FluentResource } from '@fluent/bundle';
import { mapBundleSync } from '@fluent/sequence';
import { JSX, createContext, useContext } from 'solid-js';

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

const LanguageContext = createContext(['en']);

export const LanguageProvider = (props: {
  languages: string[];
  children: JSX.Element;
}) => (
  <LanguageContext.Provider value={props.languages}>
    {props.children}
  </LanguageContext.Provider>
);

export const translate = (
  name: string,
  variables?: { [variable: string]: any },
) => {
  const languages = useContext(LanguageContext);
  const bundle = mapBundleSync(
    languages.map((language) => bundles[language]),
    name,
  );
  if (bundle == null) {
    throw new Error(`No translation found for ${name}`);
  }
  return bundle.formatPattern(bundle.getMessage(name)!.value!, variables);
};
