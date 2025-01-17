import IconButton from "./button/IconButton";
import { XMarkIcon } from "@heroicons/react/20/solid";

interface ITagProps {
  text: string;
  onClickRemove?: () => void;
  colorClassName?: string;
  outlined?: boolean;
}

/**
 * Display text as styled tag
 */
const Tag = (props: ITagProps) => {
  return (
    <div
      className={`badge ${props.colorClassName} ${props.outlined && "badge-outline"} gap-2 py-4`}
    >
      <div className="px-5">{props.text}</div>
      {props.onClickRemove && (
        <IconButton
          padding={0}
          icon={<XMarkIcon height={20} width={20} />}
          onClick={props.onClickRemove}
        />
      )}
    </div>
  );
};

export type { ITagProps };
export { Tag };
