import { createEffect, onCleanup } from 'solid-js';
import { createMutable } from 'solid-js/store';

const originalTitle = document.title;
const titleStack = createMutable([() => originalTitle]);
createEffect(() => {
  /* console.debug('Updating title stack', titleStack); */
  document.title = titleStack.map((x) => x()).join(' :: ');
});

interface TitleProps {
  children: string;
}
export const Title = (props: TitleProps) => {
  /* console.debug('Adding to title stack', props.children); */
  const getTitle = () => props.children;
  // titleStack is consumed in a reactive context, so this is valid
  // eslint-disable-next-line solid/reactivity
  titleStack.push(getTitle);
  onCleanup(() => {
    /* console.debug(`Removing from title stack`, props.children); */
    const stackEntryIndex = titleStack.indexOf(getTitle);
    if (stackEntryIndex === -1) {
      throw new Error(
        `Title stack entry for ${props.children} could not be found`,
      );
    }
    titleStack.splice(stackEntryIndex, 1);
  });
  return <></>;
};
