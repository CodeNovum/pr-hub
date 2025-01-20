import { IButtonProps, TailwindSize } from "../../types/components";
import { BusySpinner } from "../busy/BusySpinner";
import { ReactElement } from "react";

interface IIconButtonProps extends IButtonProps {
  icon: ReactElement;
  additionalClassNames?: string;
  padding?: TailwindSize;
}

/**
 * Dedicated button only responsible for displaying
 * clickable icons
 */
const IconButton = (props: IIconButtonProps) => {
  return (
    <button
      className={`btn-ghost btn flex h-max max-h-max min-h-full w-max ${props.padding !== undefined ? `p-${props.padding}` : "p-3"} ${
        props.additionalClassNames ? props.additionalClassNames : ""
      }`}
      onClick={props.onClick}
      disabled={props.disabled || props.isBusy}
    >
      {props.isBusy ? <BusySpinner /> : props.icon}
    </button>
  );
};

export default IconButton;
