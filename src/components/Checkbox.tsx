import { Label } from "./Label";

interface ICheckBoxProps {
  isChecked: boolean;
  onChange: (newValue: boolean) => void;
  label?: string;
  disabled?: boolean;
}

/**
 * Simple check box component
 */
const Checkbox = (props: ICheckBoxProps) => {
  return (
    <div className="flex max-w-max items-center">
      <input
        type="checkbox"
        checked={props.isChecked}
        className="checkbox no-animation"
        onClick={(e) => e.stopPropagation()}
        onChange={(e) => props.onChange(e.target.checked)}
        disabled={props.disabled}
      />
      {props.label && (
        <div className="ml-2">
          <Label text={props.label} />
        </div>
      )}
    </div>
  );
};

export { Checkbox };
