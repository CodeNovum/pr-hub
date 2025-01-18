import { Label } from "./Label";

type Input_Type = {
  Undefined: 0;
  Text: 1;
  Number: 2;
  Password: 3;
};
type InputType = keyof Input_Type;

interface IInputProps {
  value: string | number;
  onChange?: (newValue: string | number) => void;
  placeholder?: string;
  type?: InputType;
  label?: string;
  min?: string | number;
  disabled?: boolean;
}

/**
 * Input component for different input types including
 * type specific styling and logic
 */
const Input = (props: IInputProps) => {
  return (
    <div className="form-control w-full max-w-full">
      {props.label && props.label !== "" && <Label text={props.label} />}
      <input
        disabled={props.disabled}
        className="input-bordered input"
        type={
          props.type === "Number"
            ? "number"
            : props.type === "Password"
              ? "password"
              : "text"
        }
        min={props.min}
        value={props.value}
        onChange={(e) => props.onChange && props.onChange(e.target.value)}
        onKeyDown={(e) => {
          const isDeleteKey = e.key === "Backspace";
          const isComboKeyPressed = e.ctrlKey || e.metaKey;
          const inputAsNumber = parseInt(e.key);
          const isArrowKey =
            e.key === "ArrowLeft" ||
            e.key === "ArrowRight" ||
            e.key === "ArrowUp" ||
            e.key === "ArrowDown";
          if (
            props.type === "Number" &&
            isNaN(inputAsNumber) &&
            !isDeleteKey &&
            !isComboKeyPressed &&
            !isArrowKey
          ) {
            e.preventDefault();
            return;
          }
        }}
        placeholder={props.placeholder}
      />
    </div>
  );
};

export type { IInputProps, InputType };
export { Input };
