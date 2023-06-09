import { createEffect, onCleanup } from 'solid-js';
import { createMutable } from 'solid-js/store';

const enableDebug = false;
function debug(...data: any[]) {
  if (enableDebug) {
    console.debug('[Title]', ...data);
  }
}

const originalTitle = document.title;
const titleStack = createMutable([() => originalTitle]);
createEffect(() => {
  debug(`Updating title stack: ${titleStack}`);
  document.title = titleStack.map((x) => x()).join(' :: ');
});

interface TitleProps {
  children: string;
}
export const Title = (props: TitleProps) => {
  debug(`Adding ${props.children} to title stack`);
  const getTitle = () => props.children;
  titleStack.push(getTitle);
  onCleanup(() => {
    debug(`Removing ${props.children} from title stack`);
    const stackEntryIndex = titleStack.findIndex((elem) => elem === getTitle);
    if (stackEntryIndex === -1) {
      throw new Error(
        `Title stack entry for ${props.children} could not be found`,
      );
    }
    titleStack.splice(stackEntryIndex, 1);
  });
  return <></>;
};
