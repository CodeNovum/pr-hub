import { createRoute } from "@tanstack/react-router";
import { Root } from "./__root";
import { Organization } from "../bindings/core";
import { useMemo, useState } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { CloudArrowUpIcon, TrashIcon } from "@heroicons/react/24/outline";
import { CheckCircleIcon, XCircleIcon } from "@heroicons/react/24/solid";
import IconButton from "../components/button/IconButton";
import Table from "../components/Table";
import useOrganizations from "../hooks/useOrganizations";
import { ConfirmOrganizationDeleteDialog } from "../components/dialogs/ConfirmOrganizationDeleteDialog";
import { UpdatePatDialog } from "../components/dialogs/UpdatePatDialog";

const Organizations = createRoute({
  getParentRoute: () => Root,
  path: "/organizations",
  component: function Organizations() {
    const organizationsQuery = useOrganizations();

    const [isConfirmDeleteDialogOpen, setIsConfirmDeleteDialogOpen] =
      useState(false);
    const [selectedOrganization, setSelectedOrganization] =
      useState<Organization>();
    const [isUpdatePatDialogOpen, setIsUpdatePatDialogOpen] = useState(false);

    const tableColumns: ColumnDef<Organization>[] = useMemo(
      () => [
        {
          id: "label",
          accessorFn: (row) => row.name ?? "",
          header: "Organization",
          enableSorting: false,
        },
        {
          id: "patStatus",
          accessorKey: "isPatValid",
          header: "PAT status",
          minSize: 75,
          maxSize: 75,
          enableSorting: false,
          cell: (value) =>
            value.row.original.isPatValid ? (
              <CheckCircleIcon className="h-5 w-5 text-green-600" />
            ) : (
              <XCircleIcon className="h-5 w-5 text-red-600" />
            ),
        },
        {
          id: "actions",
          header: "Actions",
          minSize: 75,
          maxSize: 75,
          enableSorting: false,
          cell: (value) => (
            <div className="flex">
              <IconButton
                icon={<CloudArrowUpIcon className="h-5 w-5" />}
                onClick={() => {
                  setSelectedOrganization(value.row.original);
                  setIsUpdatePatDialogOpen(true);
                }}
              />
              <div className="ml-3">
                <IconButton
                  icon={<TrashIcon className="h-5 w-5" />}
                  onClick={() => {
                    setSelectedOrganization(value.row.original);
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
            isBusy={organizationsQuery.isLoading}
            columns={tableColumns}
            data={organizationsQuery.data}
          />
        </div>
        <ConfirmOrganizationDeleteDialog
          isOpen={isConfirmDeleteDialogOpen}
          close={() => setIsConfirmDeleteDialogOpen(false)}
          organization={selectedOrganization}
        />
        <UpdatePatDialog
          isOpen={isUpdatePatDialogOpen}
          close={() => setIsUpdatePatDialogOpen(false)}
          organization={selectedOrganization}
        />
      </>
    );
  },
});

export { Organizations };
