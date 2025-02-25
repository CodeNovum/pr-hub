import { GitRepositoryDto } from "../../bindings";
import { useRemoveGitRepositoryMutation } from "../../hooks/useRemoveGitRepositoryMutation";
import { BaseDialog } from "./BaseDialog";

interface IConfirmGitRepositoryDeleteDialogProps {
  isOpen: boolean;
  close: () => void;
  gitRepository?: GitRepositoryDto;
}

/**
 * Specialized dialog to let the user confirm the deletion of
 * an imported git repository
 */
const ConfirmGitRepositoryDeleteDialog = (
  props: IConfirmGitRepositoryDeleteDialogProps,
) => {
  const removeGitRepoMutation = useRemoveGitRepositoryMutation();

  return (
    <BaseDialog
      isDangerous
      isBlocking
      isOpen={props.isOpen}
      onClose={props.close}
      title="Delete git repository"
      isBusy={removeGitRepoMutation.isPending}
      onConfirm={async () => {
        if (!props.gitRepository) {
          return;
        }
        await removeGitRepoMutation.mutateAsync(props.gitRepository.id);
        props.close();
      }}
    >
      <p>Are you sure you want to delete the git repository?</p>
    </BaseDialog>
  );
};

export type { IConfirmGitRepositoryDeleteDialogProps };
export { ConfirmGitRepositoryDeleteDialog };
