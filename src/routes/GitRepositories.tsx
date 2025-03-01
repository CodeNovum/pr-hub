import { GitRepositoryDto } from "../bindings/GitRepositoryDto";
import { Checkbox } from "../components/Checkbox";
import Table from "../components/Table";
import IconButton from "../components/button/IconButton";
import { ConfirmGitRepositoryDeleteDialog } from "../components/dialogs/ConfirmGitRepositoryDeleteDialog";
import { UpdatePatDialog } from "../components/dialogs/UpdatePatDialog";
import { useRepositories } from "../hooks/useRepositories";
import { useToggleGitRepositoryActiveStateMutation } from "../hooks/useToggleGitRepositoryActiveStateMutation";
import { Root } from "./__root";
import { CloudArrowUpIcon, TrashIcon } from "@heroicons/react/24/outline";
import { createRoute } from "@tanstack/react-router";
import { ColumnDef } from "@tanstack/react-table";
import { useMemo, useState } from "react";

export const GitRepositories = createRoute({
  getParentRoute: () => Root,
  path: "/repositories",
  component: function GirRepositories() {
    const gitRepositoriesQuery = useRepositories();

    const [isConfirmDeleteDialogOpen, setIsConfirmDeleteDialogOpen] =
      useState(false);
    const [selectedGitRepository, setSelectedGitRepository] =
      useState<GitRepositoryDto>();
    const [isUpdatePatDialogOpen, setIsUpdatePatDialogOpen] = useState(false);

    const toggleActiveStateMutation =
      useToggleGitRepositoryActiveStateMutation();

    const tableColumns: ColumnDef<GitRepositoryDto>[] = useMemo(
      () => [
        {
          id: "context",
          accessorKey: "context",
          header: "Context",
          enableSorting: false,
        },
        {
          id: "name",
          accessorKey: "name",
          header: "Repository name",
          enableSorting: false,
        },
        {
          id: "gitProvider",
          accessorKey: "gitProvider",
          header: "Git Provider",
          enableSorting: false,
        },
        {
          id: "actions",
          header: "Actions",
          minSize: 75,
          maxSize: 75,
          enableSorting: false,
          cell: (value) => (
            <div className="flex">
              <Checkbox
                isChecked={value.row.original.isActive}
                onChange={async () => {
                  await toggleActiveStateMutation.mutateAsync(
                    value.row.original.id,
                  );
                }}
              />
              <div className="ml-6">
                <IconButton
                  icon={<CloudArrowUpIcon className="h-5 w-5" />}
                  onClick={() => {
                    setSelectedGitRepository(value.row.original);
                    setIsUpdatePatDialogOpen(true);
                  }}
                />
              </div>
              <div className="ml-3">
                <IconButton
                  icon={<TrashIcon className="h-5 w-5" />}
                  onClick={() => {
                    setSelectedGitRepository(value.row.original);
                    setIsConfirmDeleteDialogOpen(true);
                  }}
                />
              </div>
            </div>
          ),
        },
      ],
      [],
    );

    return (
      <>
        <div className="relative flex flex-1">
          <Table
            isBusy={gitRepositoriesQuery.isLoading}
            columns={tableColumns}
            data={gitRepositoriesQuery.data}
          />
        </div>
        <ConfirmGitRepositoryDeleteDialog
          isOpen={isConfirmDeleteDialogOpen}
          close={() => setIsConfirmDeleteDialogOpen(false)}
          gitRepository={selectedGitRepository}
        />
        <UpdatePatDialog
          isOpen={isUpdatePatDialogOpen}
          close={() => setIsUpdatePatDialogOpen(false)}
          gitRepository={selectedGitRepository}
        />
      </>
    );
  },
});
