import { ReactElement } from "react";
import { IButtonProps } from "../../types/components";
import { BusySpinner } from "../busy/BusySpinner";

interface IPrimaryButtonProps extends IButtonProps {
  text: string;
  outlined?: boolean;
  isDangerous?: boolean;
  fullWidth?: boolean;
  icon?: ReactElement;
}

/**
 * Button styled using the primary colors
 */
const PrimaryButton = (props: IPrimaryButtonProps) => {
  return (
    <button
      className={`btn ${props.outlined ? "btn-outline" : ""} ${props.isDangerous ? "btn-error" : "btn-primary"} ${props.fullWidth ? "w-full" : "w-max"}`}
      onClick={props.onClick}
      disabled={props.disabled || props.isBusy}
    >
      <div className="flex items-center justify-center gap-2">
        {props.isBusy && <BusySpinner />}
        {props.icon && !props.isBusy && props.icon}
        {props.text}
      </div>
    </button>
  );
};

export type { IPrimaryButtonProps };
export { PrimaryButton };
