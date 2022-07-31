import { FormValueType } from './form.types';
export const validate = (values: FormValueType): Partial<FormValueType> => {
  const errors: Partial<FormValueType> = {};
  if (values.name.length < 1) {
    errors.name = 'Invalid Name!';
  } else if (values.age < 0) {
    errors.age = 'Invalid Age';
  } else if (values.address.length < 1) {
    errors.address = 'Invalid address!';
  }

  return errors;
};