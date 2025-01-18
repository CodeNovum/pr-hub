import { ReactElement, Fragment } from "react";
import { Dialog, Transition } from "@headlessui/react";
import { XMarkIcon } from "@heroicons/react/24/solid";
import IconButton from "./button/IconButton";

type Panel_Size = {
  Undefined: 0;
  Small: 1;
  Medium: 2;
  Large: 3;
};
type PanelSize = keyof Panel_Size;

interface IPanelProps {
  isOpen: boolean;
  close: () => void;
  title?: string;
  isBlocking?: boolean;
  content: ReactElement;
  footer?: ReactElement;
  size?: PanelSize;
}

/**
 * Basic panel implementation
 */
const Panel = (props: IPanelProps) => {
  return (
    <Transition.Root show={props.isOpen} as={Fragment}>
      <Dialog
        as="div"
        className="relative z-10"
        onClose={() => (props.isBlocking ? null : props.close())}
      >
        <Transition.Child
          as={Fragment}
          enter="ease-in-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in-out duration-300"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
        </Transition.Child>
        <div className="fixed inset-0 overflow-hidden">
          <div className="absolute inset-0 overflow-hidden">
            <div className="pointer-events-none fixed inset-y-0 right-0 flex max-w-full">
              <Transition.Child
                as={Fragment}
                enter="transform transition ease-in-out duration-300"
                enterFrom="translate-x-full"
                enterTo="translate-x-0"
                leave="transform transition ease-in-out duration-300"
                leaveFrom="translate-x-0"
                leaveTo="translate-x-full"
              >
                <Dialog.Panel
                  className={`pointer-events-auto relative w-screen bg-base-100 ${
                    props.size === "Large"
                      ? "sm:max-w-2xl"
                      : props.size === "Medium"
                        ? "sm:max-w-lg"
                        : "sm:max-w-md"
                  }`}
                >
                  <div className="absolute top-3 right-3 z-50">
                    <IconButton
                      icon={<XMarkIcon className="h-5 w-5" />}
                      onClick={props.close}
                    />
                  </div>
                  <div className="flex h-full flex-col">
                    {props.title && (
                      <div className="px-5 py-3">
                        <Dialog.Title>{props.title}</Dialog.Title>
                      </div>
                    )}
                    <div className="relative flex h-full flex-1 flex-col overflow-auto px-5">
                      {props.content}
                    </div>
                    {props.footer && (
                      <div className="mt-auto flex px-5 py-3">
                        {props.footer}
                      </div>
                    )}
                  </div>
                </Dialog.Panel>
              </Transition.Child>
            </div>
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  );
};

export type { PanelSize, IPanelProps };
export { Panel };
