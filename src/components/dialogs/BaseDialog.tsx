import { DefaultButton } from "../button/DefaultButton";
import { PrimaryButton } from "../button/PrimaryButton";
import { ReactNode } from "react";

interface IDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm?: () => void;
  title: string;
  children?: ReactNode;
  isBlocking?: boolean;
  isDangerous?: boolean;
  cancelText?: string;
  confirmText?: string;
  isBusy?: boolean;
}
/**
 * Basic dialog implementation
 */
const BaseDialog = (props: IDialogProps) => {
  return (
    <div
      className={`modal ${props.isOpen ? "modal-open bg-gray-500 bg-opacity-75 transition-opacity" : ""}`}
      onClick={() => (props.isBlocking ? null : props.onClose())}
    >
      <div className="modal-box" onClick={(e) => e.stopPropagation()}>
        <h3 className="text-lg font-bold">{props.title}</h3>
        <div className="py-4">{props.children}</div>
        <div className="modal-action">
          <DefaultButton
            text={props.cancelText ? props.cancelText : "Close"}
            onClick={props.onClose}
          />
          {props.onConfirm && (
            <PrimaryButton
              isBusy={props.isBusy}
              isDangerous={props.isDangerous}
              outlined
              text={props.confirmText ? props.confirmText : "Confirm"}
              onClick={props.onConfirm}
            />
          )}
        </div>
      </div>
    </div>
  );
};

export type { IDialogProps };
export { BaseDialog };
