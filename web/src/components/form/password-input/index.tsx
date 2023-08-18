import { TextInput, TextInputProps } from '@/components/form/text-input';
import { translate } from '@/localization';

export type PasswordInputProps = Omit<TextInputProps, 'placeholder'>;

export const PasswordInput = (props: PasswordInputProps) => {
  return (
    <TextInput
      placeholder={translate('login--password')}
      type='password'
      {...props}
    />
  );
};
