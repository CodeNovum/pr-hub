import { DarkModeToggle } from "../components/DarkModeToggle";
import { PrimaryButton } from "../components/button/PrimaryButton";
import { AddAzureDevOpsOrganizationPanel } from "../components/panels/AddAzureDevOpsOrganizationPanel";
import { PlusIcon } from "@heroicons/react/24/solid";
import { Link, Outlet, createRootRoute } from "@tanstack/react-router";
import { useState } from "react";

const Root = createRootRoute({
  component: () => {
    const [isCreatePanelOpen, setIsCreatePanelOpen] = useState(false);

    return (
      <>
        <div className="flex max-h-screen h-screen flex-1 flex-col">
          <div className="p-2 grid grid-cols-3">
            <div className="flex gap-2 items-center justify-start">
              <Link to="/" className="[&.active]:font-bold">
                Pull Requests
              </Link>{" "}
              <Link to="/repositories" className="[&.active]:font-bold">
                Repositories
              </Link>
            </div>
            <div className="flex items-center justify-center">
              <PrimaryButton
                outlined
                icon={<PlusIcon className="h-5 w-5" />}
                text={"Azure DevOps organization"}
                onClick={() => setIsCreatePanelOpen(true)}
              />
            </div>
            <div className="flex items-center justify-end gap-2">
              <DarkModeToggle />
            </div>
          </div>
          <hr />
          <div className="flex flex-1 overflow-hidden">
            <Outlet />
          </div>
        </div>
        <AddAzureDevOpsOrganizationPanel
          isOpen={isCreatePanelOpen}
          close={() => setIsCreatePanelOpen(false)}
        />
      </>
    );
  },
});

export { Root };
