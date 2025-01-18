import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";
import {
  MutableRefObject,
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import {
  ArrowSmallDownIcon,
  ArrowSmallUpIcon,
} from "@heroicons/react/24/solid";
import { BusySpinnerOverlay } from "./busy/BusySpinnerOverlay";
import { Checkbox } from "./Checkbox";
import { useIsDarkModeEnabled } from "../hooks/useIsDarkModeEnabled";

interface ITableProps<TData> {
  columns: ColumnDef<TData>[];
  data?: TData[];
  isBusy?: boolean;
  identifierPropertyName?: string;
  onRowClick?: (item: TData) => void;
  checkedItems?: TData[];
  updateCheckedItems?: (newCheckedItems: TData[]) => void;
}

/**
 * Generic, styled table that reacts to theme switches and provides
 * the possibility to define callbacks to interact with the rows
 */
const Table = <TData,>(props: ITableProps<TData>) => {
  const selectColumnWidth = 56;
  const isDarkModeEnabled = useIsDarkModeEnabled();

  const [lastClickedRowItem, setLastClickedRowItem] = useState<TData>();
  const [isHorizontalScrollUsed, setIsHorizontalScrollUsed] = useState(false);

  const scrollableContainer = useRef<HTMLDivElement>();

  const data = useMemo(() => (props.data ? props.data : []), [props.data]);
  const columns = useMemo(() => {
    // TODO: Find a way to disable the re-render for the whole table on select.
    let cols = props.columns;
    if (
      props.updateCheckedItems &&
      props.identifierPropertyName &&
      props.checkedItems
    ) {
      const selectCol: ColumnDef<TData> = {
        id: "select",
        cell: (cellProps) => {
          // Create copy of currently checked items to work on.
          const tmpCheckedItems = props.checkedItems
            ? [...props.checkedItems]
            : [];
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          const obj = cellProps.row.original as any;
          const identifier = obj[props.identifierPropertyName as string];
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          const index = tmpCheckedItems.findIndex(
            (i) =>
              (i as any)[props.identifierPropertyName as string] === identifier,
          );
          return (
            <Checkbox
              isChecked={index !== -1}
              onChange={() => {
                if (index === -1) {
                  tmpCheckedItems.push(obj);
                } else {
                  // Remove the item from the internal table state of checked item identifiers.
                  tmpCheckedItems.splice(index, 1);
                }
                if (props.updateCheckedItems) {
                  props.updateCheckedItems([...tmpCheckedItems]);
                }
              }}
            />
          );
        },
        minSize: selectColumnWidth,
        maxSize: selectColumnWidth,
      };
      cols = [selectCol, ...cols];
    }
    return cols;
  }, [props]);

  const table = useReactTable({
    columns: columns,
    data: data,
    getCoreRowModel: getCoreRowModel(),
  });

  useEffect(() => {
    const errorMessage = "Invalid table property combination.";
    if (
      props.updateCheckedItems ||
      props.identifierPropertyName ||
      props.checkedItems
    ) {
      if (
        !props.updateCheckedItems ||
        !props.identifierPropertyName ||
        !props.checkedItems
      ) {
        throw new Error(errorMessage);
      }
    }
  }, [
    props.checkedItems,
    props.data,
    props.identifierPropertyName,
    props.updateCheckedItems,
  ]);

  const displayScrollSeparator = useCallback(
    (i: number) =>
      isHorizontalScrollUsed &&
      ((i === 0 &&
        !props.identifierPropertyName &&
        !props.updateCheckedItems) ||
        (i === 1 && props.identifierPropertyName && props.updateCheckedItems)),
    [
      isHorizontalScrollUsed,
      props.identifierPropertyName,
      props.updateCheckedItems,
    ],
  );

  return (
    <div className="absolute top-0 left-0 flex h-full w-full overflow-hidden">
      <div className="flex flex-1 flex-col overflow-hidden">
        {props.isBusy && <BusySpinnerOverlay />}
        <div
          onScroll={() => {
            const currentHorizontalScroll =
              scrollableContainer.current?.scrollLeft ?? 0;
            setIsHorizontalScrollUsed(currentHorizontalScroll > 0);
          }}
          ref={scrollableContainer as MutableRefObject<HTMLDivElement>}
          className="sticky top-0 left-0 h-full w-full overflow-auto"
        >
          <table className="table w-full">
            <thead className="sticky top-0 z-20">
              {table.getHeaderGroups().map((headerGroup) => (
                <tr key={headerGroup.id}>
                  {headerGroup.headers.map((header, i) => (
                    <th
                      key={header.id}
                      style={{
                        width: header.getSize(),
                        minWidth: header.getSize(),
                        maxWidth: header.column.columnDef.maxSize,
                      }}
                      className={`sticky top-0 bg-white p-0 dark:bg-base-100 ${
                        i === 0
                          ? "left-0 z-10"
                          : i === 1 &&
                              props.identifierPropertyName &&
                              props.updateCheckedItems
                            ? "left-14 z-20"
                            : ""
                      }`}
                    >
                      {header.isPlaceholder ? null : (
                        <div
                          className={`relative min-h-[50px] py-0 px-4 ${
                            header.column.getIsSorted()
                              ? "font-bold"
                              : "font-normal"
                          } border-b border-t bg-base-100 ${
                            isDarkModeEnabled
                              ? "border-border-gray"
                              : "border-base-200"
                          } flex w-full flex-1 items-center justify-start ${header.column.getCanSort() ? "cursor-pointer" : ""} min-w-max`}
                          onClick={() => {
                            if (!header.column.getCanSort()) {
                              return;
                            }
                            header.column.toggleSorting();
                          }}
                        >
                          {flexRender(
                            header.column.columnDef.header,
                            header.getContext(),
                          )}
                          <span
                            className={`absolute top-0 right-0 h-full border-r duration-500  ${
                              displayScrollSeparator(i)
                                ? isDarkModeEnabled
                                  ? "border-border-gray"
                                  : "border-base-200"
                                : "border-transparent"
                            }`}
                          />
                          <div className="ml-2">
                            {{
                              asc: <ArrowSmallUpIcon className="h-4 w-4" />,
                              desc: <ArrowSmallDownIcon className="h-4 w-4" />,
                            }[header.column.getIsSorted() as string] ?? null}
                          </div>
                        </div>
                      )}
                    </th>
                  ))}
                </tr>
              ))}
            </thead>
            <tbody>
              {table.getRowModel().rows.map((row) => (
                <tr
                  className={`text-sm ${props.onRowClick ? "cursor-pointer hover" : ""} ${
                    lastClickedRowItem === row.original
                      ? isDarkModeEnabled
                        ? "text-secondary"
                        : "text-primary"
                      : ""
                  }`}
                  key={row.id}
                  onClick={() => {
                    if (props.onRowClick) {
                      setLastClickedRowItem(row.original);
                      props.onRowClick(row.original);
                    }
                  }}
                >
                  {row.getVisibleCells().map((cell, i) => (
                    <td
                      style={{
                        width: cell.column.columnDef.minSize,
                        minWidth: cell.column.columnDef.minSize,
                        maxWidth: cell.column.columnDef.maxSize,
                      }}
                      key={cell.id}
                      className={`relative p-4 ${
                        isDarkModeEnabled ? "bg-base-100" : "bg-white"
                      } overflow-hidden text-ellipsis whitespace-nowrap border-none ${
                        i === 0
                          ? "sticky left-0 z-10"
                          : i === 1 &&
                              props.identifierPropertyName &&
                              props.updateCheckedItems
                            ? "sticky left-14 z-10"
                            : ""
                      }`}
                    >
                      <>
                        {flexRender(
                          cell.column.columnDef.cell,
                          cell.getContext(),
                        )}
                        <span
                          className={`absolute top-0 right-0 h-full border-r duration-500  ${
                            displayScrollSeparator(i)
                              ? isDarkModeEnabled
                                ? "border-border-gray"
                                : "border-base-200"
                              : "border-transparent"
                          }`}
                        />
                        <span
                          className={`absolute bottom-0 left-0 w-full border-b ${isDarkModeEnabled ? "border-border-gray" : "border-base-200"}`}
                        />
                      </>
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
            <tfoot>
              {table.getFooterGroups().map((footerGroup) => (
                <tr key={footerGroup.id}>
                  {footerGroup.headers.map((header) => (
                    <th className="bg-base-100" key={header.id}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                            header.column.columnDef.footer,
                            header.getContext(),
                          )}
                    </th>
                  ))}
                </tr>
              ))}
            </tfoot>
          </table>
        </div>
      </div>
    </div>
  );
};

export default Table;
