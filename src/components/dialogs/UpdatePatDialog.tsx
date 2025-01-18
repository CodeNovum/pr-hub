import { Organization } from "../../bindings/core";
import { useUpdatePatMutation } from "../../hooks/useUpdatePatMutation";
import { Input } from "../Input";
import { Stack } from "../Stack";
import { BaseDialog } from "./BaseDialog";
import { useState } from "react";

interface IUpdatePatDialogProps {
  isOpen: boolean;
  close: () => void;
  organization?: Organization;
}

/**
 * Specialized dialog for the user to update the PAT of a
 * single imported organization
 */
const UpdatePatDialog = (props: IUpdatePatDialogProps) => {
  const updatePatMutation = useUpdatePatMutation();

  const [patValue, setPatValue] = useState("");

  const closeDialog = () => {
    setPatValue("");
    props.close();
  };

  return (
    <BaseDialog
      title="Update PAT"
      isOpen={props.isOpen}
      onClose={closeDialog}
      isBlocking
      isBusy={updatePatMutation.isPending}
      onConfirm={async () => {
        if (!props.organization) {
          return;
        }
        const updatedOrganization = { ...props.organization };
        updatedOrganization.pat = patValue;
        await updatePatMutation.mutateAsync(updatedOrganization);
        closeDialog();
      }}
    >
      <Stack>
        <p>
          Update Personal Access Token for the currently selected organization
        </p>
        <Input
          label="Personal Access Token"
          value={patValue}
          onChange={(newValue) => setPatValue(newValue as string)}
          type="Password"
        />
      </Stack>
    </BaseDialog>
  );
};

export type { IUpdatePatDialogProps };
export { UpdatePatDialog };
