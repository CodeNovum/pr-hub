import { Organization } from "../../bindings/core";
import { useRemoveOrganizationMutation } from "../../hooks/useRemoveOrganizationMutation";
import { BaseDialog } from "./BaseDialog";

interface IConfirmOrganizationDeleteDialogProps {
  isOpen: boolean;
  close: () => void;
  organization?: Organization;
}

/**
 * Specialized dialog to let the user confirm the deletion of
 * an imported organization
 */
const ConfirmOrganizationDeleteDialog = (
  props: IConfirmOrganizationDeleteDialogProps,
) => {
  const removeOrganizationMutation = useRemoveOrganizationMutation();

  return (
    <BaseDialog
      isDangerous
      isBlocking
      isOpen={props.isOpen}
      onClose={props.close}
      title="Delete organization"
      isBusy={removeOrganizationMutation.isPending}
      onConfirm={async () => {
        if (!props.organization) {
          return;
        }
        await removeOrganizationMutation.mutateAsync(props.organization.id);
        props.close();
      }}
    >
      <p>Are you sure you want to delete the organization?</p>
    </BaseDialog>
  );
};

export type { IConfirmOrganizationDeleteDialogProps };
export { ConfirmOrganizationDeleteDialog };
