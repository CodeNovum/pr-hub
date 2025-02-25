import { PullRequestDto } from "../bindings";
import Table from "../components/Table";
import { usePullRequests } from "../hooks/usePullRequests";
import { Root } from "./__root";
import { createRoute } from "@tanstack/react-router";
import { ColumnDef } from "@tanstack/react-table";
import { format } from "date-fns/format";
import { useMemo } from "react";

const PullRequests = createRoute({
  getParentRoute: () => Root,
  path: "/",
  component: function PullRequests() {
    const pullRequestsQuery = usePullRequests();

    const tableColumns: ColumnDef<PullRequestDto>[] = useMemo(
      () => [
        {
          id: "repositoryName",
          header: "Repository",
          accessorKey: "repositoryName",
          enableSorting: false,
        },
        {
          id: "title",
          accessorKey: "title",
          header: "Title",
          enableSorting: false,
        },
        {
          id: "mergeStatus",
          accessorKey: "mergeStatus",
          header: "Merge Status",
          enableSorting: false,
        },
        {
          id: "creator",
          header: "Creator",
          accessorKey: "creatorName",
          enableSorting: false,
        },
        {
          id: "creationDate",
          header: "Open since",
          enableSorting: false,
          cell: (value) =>
            value.row.original.creationDate
              ? format(new Date(value.row.original.creationDate), "dd.MM.yyyy")
              : "",
        },
      ],
      [],
    );

    return (
      <div className="relative flex flex-1">
        <Table
          isBusy={pullRequestsQuery.isLoading}
          columns={tableColumns}
          data={pullRequestsQuery.data?.sort((a, b) =>
            a.creationDate && b.creationDate && a.creationDate > b.creationDate
              ? 1
              : -1,
          )}
        />
      </div>
    );
  },
});

export { PullRequests };
