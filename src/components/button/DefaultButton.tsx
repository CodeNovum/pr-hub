import { IButtonProps } from "../../types/components";
import { BusySpinner } from "../busy/BusySpinner";

interface IDefaultButtonProps extends IButtonProps {
  text: string;
  fullWidth?: boolean;
}

/**
 * Button using the default colors for styling
 */
const DefaultButton = (props: IDefaultButtonProps) => {
  return (
    <button
      className={`btn-ghost btn ${props.fullWidth ? "w-full" : "w-max"}`}
      onClick={props.onClick}
      disabled={props.disabled || props.isBusy}
    >
      <div className="flex items-center justify-center gap-2">
        {props.isBusy && <BusySpinner />}
        {props.text}
      </div>
    </button>
  );
};

export type { IDefaultButtonProps };
export { DefaultButton };
