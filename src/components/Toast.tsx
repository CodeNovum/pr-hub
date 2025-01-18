import { XMarkIcon } from "@heroicons/react/24/solid";
import IconButton from "./button/IconButton";

const Toast_Type = {
  Undefined: 0,
  Info: 1,
  Warning: 2,
  Error: 3,
  Success: 4,
} as const;
type ToastType = keyof typeof Toast_Type;

interface IToastProps {
  type: ToastType;
  message: string;
  dismiss?: () => void;
}

/**
 * Display text as toast globally
 */
const Toast = (props: IToastProps) => (
  <div className="toast-center toast w-1/2">
    <div
      className={`alert flex py-2 ${
        props.type === "Success"
          ? "alert-success"
          : props.type === "Error"
            ? "alert-error"
            : props.type === "Warning"
              ? "alert-warning"
              : "alert-info"
      }`}
    >
      <div className="flex w-full items-center justify-between">
        <span>{props.message}</span>
        {props.dismiss && (
          <IconButton
            icon={<XMarkIcon className="h-4 w-4" />}
            onClick={props.dismiss}
          />
        )}
      </div>
    </div>
  </div>
);

export type { ToastType };
export { Toast };
