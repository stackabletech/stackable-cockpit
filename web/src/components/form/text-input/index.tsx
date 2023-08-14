import { JSX } from 'solid-js';
import styles from './text-input.module.scss';

export interface TextInputProps
  extends JSX.InputHTMLAttributes<HTMLInputElement> {
  placeholder: string;
}

export const TextInput = (props: TextInputProps) => {
  return <input class={styles.textInput} {...props} />;
};
