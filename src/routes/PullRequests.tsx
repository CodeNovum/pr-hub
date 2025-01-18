import { DevOpsUser, Label, PullRequest } from "../bindings/devops";
import { Panel } from "../components/Panel";
import { Stack } from "../components/Stack";
import Table from "../components/Table";
import { Tag } from "../components/Tag";
import IconButton from "../components/button/IconButton";
import { usePullRequests } from "../hooks/usePullRequests";
import { Root } from "./__root";
import { LinkIcon } from "@heroicons/react/24/outline";
import { ArrowLeftOnRectangleIcon } from "@heroicons/react/24/solid";
import { createRoute } from "@tanstack/react-router";
import { ColumnDef } from "@tanstack/react-table";
import { open } from "@tauri-apps/plugin-shell";
import { format } from "date-fns/format";
import { useMemo, useState } from "react";

const PullRequests = createRoute({
  getParentRoute: () => Root,
  path: "/",
  component: function PullRequests() {
    const pullRequestsQuery = usePullRequests();

    const [isTagsPanelOpen, setIsTagsPanelOpen] = useState(false);
    const [isReviewersPanelOpen, setIsReviewersPanelOpen] = useState(false);
    const [selectedPullRequest, setSelectedPullRequest] =
      useState<PullRequest>();

    const openDevOpsPullRequestInNewTab = (pullRequest: PullRequest) => {
      if (
        !pullRequest.repository?.name ||
        !pullRequest.repository?.project?.name ||
        !pullRequest.organizationName ||
        !pullRequest.pullRequestId
      ) {
        return;
      }
      open(
        `https://dev.azure.com/${pullRequest.organizationName}/${pullRequest.repository.project.name}/_git/${pullRequest.repository.name}/pullrequest/${pullRequest.pullRequestId}`,
      );
    };

    const tableColumns: ColumnDef<PullRequest>[] = useMemo(
      () => [
        {
          id: "project",
          header: "Project",
          enableSorting: false,
          accessorFn: (value) => value.repository?.project?.name ?? "",
        },
        {
          id: "repository",
          header: "Repository",
          enableSorting: false,
          accessorFn: (value) => value.repository?.name ?? "",
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
          id: "status",
          accessorKey: "status",
          header: "Status",
          enableSorting: false,
        },
        {
          id: "creator",
          header: "Creator",
          enableSorting: false,
          accessorFn: (value) => value.createdBy?.displayName ?? "",
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
        {
          id: "fixedComments",
          header: "Fixed comments",
          enableSorting: false,
          cell: (value) => {
            const commentThreads = value.row.original.commentThreads;
            const resolvedCommentsCount =
              commentThreads?.filter(
                (thread) =>
                  (thread.status === "closed" ||
                    thread.status === "fixed" ||
                    thread.status === "wontFix" ||
                    thread.status === "byDesign") &&
                  thread.comments?.some((c) => c.commentType === "text"),
              ).length ?? 0;
            const allCommentsCount =
              commentThreads?.filter((thread) =>
                thread.comments?.some((c) => c.commentType === "text"),
              ).length ?? 0;
            return (
              <div
                className={`${resolvedCommentsCount && allCommentsCount && resolvedCommentsCount < allCommentsCount ? "text-red-600" : "text-green-600"} z-0`}
              >
                {`${resolvedCommentsCount ?? 0} / ${allCommentsCount ?? 0}`}
              </div>
            );
          },
        },
        {
          id: "tags",
          accessorKey: "",
          header: "Tags",
          enableSorting: false,
          cell: (value) => (
            <IconButton
              icon={<ArrowLeftOnRectangleIcon className="h-5 w-5" />}
              onClick={() => {
                setSelectedPullRequest(value.row.original);
                setIsTagsPanelOpen(true);
              }}
            />
          ),
        },
        {
          id: "reviewers",
          accessorKey: "",
          header: "Reviewers",
          enableSorting: false,
          cell: (value) => (
            <IconButton
              icon={<ArrowLeftOnRectangleIcon className="h-5 w-5" />}
              onClick={() => {
                setSelectedPullRequest(value.row.original);
                setIsReviewersPanelOpen(true);
              }}
            />
          ),
        },
        {
          id: "actions",
          header: "Actions",
          minSize: 75,
          maxSize: 75,
          enableSorting: false,
          cell: (value) => (
            <IconButton
              icon={<LinkIcon className="h-5 w-5" />}
              onClick={() => openDevOpsPullRequestInNewTab(value.row.original)}
            />
          ),
        },
      ],
      [],
    );

    return (
      <>
        <div className="relative flex flex-1">
          <Table
            isBusy={pullRequestsQuery.isLoading}
            columns={tableColumns}
            data={pullRequestsQuery.data?.sort((a, b) =>
              a.creationDate &&
              b.creationDate &&
              a.creationDate > b.creationDate
                ? 1
                : -1,
            )}
          />
        </div>
        <Panel
          isOpen={isTagsPanelOpen}
          close={() => setIsTagsPanelOpen(false)}
          title="Tags"
          content={
            <Stack horizontal>
              {selectedPullRequest?.labels?.map((label: Label) => (
                <Tag
                  key={`pull-request-tag-${label.id}-${label.name}`}
                  text={label.name ?? ""}
                />
              ))}
            </Stack>
          }
        />
        <Panel
          isOpen={isReviewersPanelOpen}
          close={() => setIsReviewersPanelOpen(false)}
          title="Reviewers"
          content={
            <Stack horizontal>
              {selectedPullRequest?.reviewers?.map((reviewer: DevOpsUser) => (
                <Tag
                  key={`pull-request-tag-${reviewer.id}-${reviewer.displayName}`}
                  text={reviewer.displayName ?? ""}
                />
              ))}
            </Stack>
          }
        />
      </>
    );
  },
});

export { PullRequests };
