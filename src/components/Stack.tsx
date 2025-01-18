import { MutableRefObject, ReactNode, useMemo, useRef } from "react";
import { TailwindSize } from "../types/components";

type Stack_Vertical_Align = {
  Undefined: 0;
  Start: 1;
  End: 2;
  Center: 3;
  Baseline: 4;
  Stretch: 5;
};
type StackVerticalAlign = keyof Stack_Vertical_Align;

type Stack_Horizontal_Align = {
  Undefined: 0;
  Start: 1;
  End: 2;
  Center: 3;
  SpaceBetween: 4;
  SpaceAround: 5;
  SpaceEvenly: 6;
};
type StackHorizontalAlign = keyof Stack_Horizontal_Align;

interface IStackProps {
  children: ReactNode;
  horizontal?: boolean;
  wrap?: boolean;
  horizontalAlign?: StackHorizontalAlign;
  verticalAlign?: StackVerticalAlign;
  height?: TailwindSize;
  width?: TailwindSize;
  maxHeight?: TailwindSize;
  maxWidth?: TailwindSize;
  gapSize?: TailwindSize;
  paddingSize?: TailwindSize;
}

/**
 * Easy layout for children
 */
const Stack = (props: IStackProps) => {
  const stackRef = useRef<HTMLDivElement>();

  const verticalAlignClassName = useMemo((): string => {
    switch (props.verticalAlign) {
      case "Baseline":
        return props.horizontal ? "items-baseline" : "justify-start";
      case "Center":
        return props.horizontal ? "items-center" : "justify-center";
      case "End":
        return props.horizontal ? "items-end" : "justify-end";
      case "Start":
        return props.horizontal ? "items-start" : "justify-start";
      case "Stretch":
        return "items-stretch";
      default:
        return "";
    }
  }, [props.horizontal, props.verticalAlign]);

  const horizontalAlignClassName = useMemo((): string => {
    switch (props.horizontalAlign) {
      case "Center":
        return props.horizontal ? "justify-center" : "items-center";
      case "End":
        return props.horizontal ? "justify-end" : "items-end";
      case "SpaceAround":
        return "justify-around";
      case "SpaceBetween":
        return "justify-between";
      case "SpaceEvenly":
        return "justify-evenly";
      case "Start":
        return props.horizontal ? "justify-start" : "items-start";
      default:
        return "";
    }
  }, [props.horizontal, props.horizontalAlign]);

  const gapClassName = useMemo((): string => {
    const className = props.gapSize != null ? `gap-${props.gapSize}` : "gap-4";
    return className;
  }, [props.gapSize]);

  const paddingClassName = useMemo((): string => {
    const className =
      props.paddingSize != null ? `p-${props.paddingSize}` : "p-0";
    return className;
  }, [props.paddingSize]);

  const widthClassName = useMemo((): string => {
    const className = props.width != null ? `w-${props.width}` : "";
    return className;
  }, [props.width]);

  const heightClassName = useMemo((): string => {
    const className = props.height != null ? `h-${props.height}` : "";
    return className;
  }, [props.height]);

  const maxWidthClassName = useMemo((): string => {
    const className = props.maxWidth != null ? `max-w-${props.maxWidth}` : "";
    return className;
  }, [props.maxWidth]);

  const maxHeightClassName = useMemo((): string => {
    const className = props.maxHeight != null ? `max-h-${props.maxHeight}` : "";
    return className;
  }, [props.maxHeight]);

  return (
    <div
      ref={stackRef as MutableRefObject<HTMLDivElement>}
      className={`flex ${props.horizontal ? "flex-row" : "flex-col"} ${
        props.wrap ? "flex-wrap" : ""
      } ${verticalAlignClassName} ${horizontalAlignClassName} ${gapClassName} ${paddingClassName} 
            ${widthClassName} ${heightClassName} ${maxWidthClassName} ${maxHeightClassName}`}
    >
      {props.children}
    </div>
  );
};

export type { IStackProps, StackHorizontalAlign, StackVerticalAlign };
export { Stack };
