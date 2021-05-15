import { ChangeEvent, useState } from "react";

// I dreamt about this hook on 11/12!2020 and woke up to try it out at 4am.
// This is how I'll be dealing with form in React moving forward till I find a better way

// Todo: Make the hook cover Checkbox and some other edge cases

/* interface InputValues<ThisLabelName, AllLabelNames> {
  getInputAttributes: (inputName: keyof AllLabelNames) => {
    onChange: (event: ChangeEvent<HTMLInputElement>) => void;
    name: keyof AllLabelNames;
    value: AllLabelNames[keyof AllLabelNames];
    error:
      | NonNullable<
          Record<keyof AllLabelNames, string | boolean | null>
        >[keyof AllLabelNames]
      | undefined;
    reset: () => void;
  };
  inputs: AllLabelNames;
  errors:
    | Record<keyof AllLabelNames, string | boolean | null>
    | null
    | undefined;
  resetAllFields: () => void;
  noErrors: boolean;
} */

const useForm = <T>(
  initialValues: T,
  validateFn: (
    inputsValues: Readonly<T>
  ) => Record<keyof T, string | null | boolean> | null = () => null
) => {
  const [inputs, setInputs] = useState(initialValues);
  const [errors, setErrors] = useState<ReturnType<typeof validateFn>>();

  const handleInputChange = (event: ChangeEvent<HTMLInputElement>) => {
    event.persist();

    setInputs((prevState) => {
      const newState = {
        ...prevState,
        [event.target.name]: event.target.value,
      };
      const validationErrors = validateFn(newState);
      setErrors(validationErrors);

      return newState;
    });
  };

  const resetField = (inputName: keyof T) => {
    setInputs((prevState) => {
      const newState = {
        ...prevState,
        [inputName]: initialValues[inputName],
      };
      const validationErrors = validateFn(newState);
      setErrors(validationErrors);

      return newState;
    });
  };

  const getInputAttributes = <ThisLabelName extends keyof T>(inputName: ThisLabelName) => {
    return {
      onChange: handleInputChange,
      name: inputName,
      value: inputs[inputName],
      error: errors?.[inputName],
      reset: () => resetField(inputName),
    };
  };

  const resetAllFields = () => {
    setInputs(initialValues);
  };

  return {
    getInputAttributes,
    inputs,
    errors,
    resetAllFields,
    noErrors: !(
      errors && Object.values(errors).some((e) => isString(e) && e.length > 0)
    ),
  };
};

export default useForm;

function isString(str: unknown): str is string {
  return (
    typeof str === "string" ||
    (!!str &&
      typeof str === "object" &&
      Object.prototype.toString.call(str) === "[object String]")
  );
}
