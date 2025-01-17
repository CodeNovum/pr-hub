import { createRootRoute, Link, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { GlobalProjectFilterPanel } from "../components/panels/GlobalProjectFilterPanel";
import { useState } from "react";
import IconButton from "../components/button/IconButton";
import { CogIcon } from "@heroicons/react/24/outline";
import { PlusIcon } from "@heroicons/react/24/solid";
import { DarkModeToggle } from "../components/DarkModeToggle";
import { PrimaryButton } from "../components/button/PrimaryButton";
import { AddOrganizationPanel } from "../components/panels/AddOrganizationPanel";

const Root = createRootRoute({
  component: () => {
    const [isProjectFilterOpen, setIsProjectFilterOpen] = useState(false);
    const [isCreatePanelOpen, setIsCreatePanelOpen] = useState(false);

    return (
      <>
        <div className="flex max-h-screen h-screen flex-1 flex-col">
          <div className="p-2 grid grid-cols-3">
            <div className="flex gap-2 items-center justify-start">
              <Link to="/" className="[&.active]:font-bold">
                Pull Requests
              </Link>{" "}
              <Link to="/organizations" className="[&.active]:font-bold">
                Organizations
              </Link>
            </div>
            <div className="flex items-center justify-center">
              <PrimaryButton
                outlined
                icon={<PlusIcon className="h-5 w-5" />}
                text={"DevOps organization"}
                onClick={() => setIsCreatePanelOpen(true)}
              />
            </div>
            <div className="flex items-center justify-end gap-2">
              <IconButton
                icon={<CogIcon className="h-5 w-5" />}
                onClick={() => setIsProjectFilterOpen(true)}
              />
              <DarkModeToggle />
            </div>
          </div>
          <hr />
          <div className="flex flex-1 overflow-hidden">
            <Outlet />
          </div>
          <TanStackRouterDevtools />
        </div>
        <GlobalProjectFilterPanel
          isOpen={isProjectFilterOpen}
          close={() => setIsProjectFilterOpen(false)}
        />
        <AddOrganizationPanel
          isOpen={isCreatePanelOpen}
          close={() => setIsCreatePanelOpen(false)}
        />
      </>
    );
  },
});

export { Root };
