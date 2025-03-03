import { GitRepositoryDto } from "../../bindings/GitRepositoryDto";
import { useUpdatePatMutation } from "../../hooks/useUpdatePatMutation";
import { Input } from "../Input";
import { Stack } from "../Stack";
import { BaseDialog } from "./BaseDialog";
import { useState } from "react";

interface IUpdatePatDialogProps {
  isOpen: boolean;
  close: () => void;
  gitRepository?: GitRepositoryDto;
}

/**
 * Specialized dialog for the user to update the PAT of a
 * single imported git repository
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
        if (!props.gitRepository) {
          return;
        }
        await updatePatMutation.mutateAsync({
          id: props.gitRepository.id,
          pat: patValue,
        });
        closeDialog();
      }}
    >
      <Stack>
        <p>
          Update Personal Access Token for the currently selected git repository
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
