import { useAddOrganizationMutation } from "../../hooks/useAddOrganizationMutation";
import { Input } from "../Input";
import { Panel } from "../Panel";
import { Stack } from "../Stack";
import { PrimaryButton } from "../button/PrimaryButton";
import { useCallback, useMemo, useState } from "react";

interface IAddOrganizationPanelProps {
  isOpen: boolean;
  close: () => void;
}

/**
 * Specialized panel that enables the user to import a new
 * organization and the corresponding PAT
 */
const AddOrganizationPanel = (props: IAddOrganizationPanelProps) => {
  const addOrganizationMutation = useAddOrganizationMutation();

  const [organizationName, setOrganizationName] = useState<string>("");
  const [personalAccessToken, setPersonalAccessToken] = useState<string>("");

  const isSubmitEnabled = useMemo(
    () =>
      organizationName &&
      organizationName !== "" &&
      personalAccessToken &&
      personalAccessToken !== ""
        ? true
        : false,
    [organizationName, personalAccessToken],
  );

  const closePanel = useCallback(() => {
    setOrganizationName("");
    setPersonalAccessToken("");
    props.close();
  }, [props]);

  const body = useMemo(
    () => (
      <Stack>
        <Input
          label="Organization name"
          value={organizationName}
          onChange={(newValue) => setOrganizationName(newValue as string)}
        />
        <Input
          type="Password"
          label="Personal Access Token"
          value={personalAccessToken}
          onChange={(newValue) => setPersonalAccessToken(newValue as string)}
        />
      </Stack>
    ),
    [organizationName, personalAccessToken],
  );

  const footer = useMemo(
    () => (
      <div className="w-full">
        <Stack horizontal horizontalAlign="End">
          <PrimaryButton
            disabled={!isSubmitEnabled || addOrganizationMutation.isPending}
            isBusy={addOrganizationMutation.isPending}
            text="Add"
            onClick={async () => {
              await addOrganizationMutation.mutateAsync({
                organizationName: organizationName,
                personalAccessTokenValue: personalAccessToken,
              });
              closePanel();
            }}
          />
        </Stack>
      </div>
    ),
    [
      addOrganizationMutation,
      closePanel,
      isSubmitEnabled,
      organizationName,
      personalAccessToken,
    ],
  );

  return (
    <Panel
      title="Add DevOps"
      close={closePanel}
      isOpen={props.isOpen}
      content={body}
      footer={footer}
      isBlocking
    />
  );
};

export type { IAddOrganizationPanelProps };
export { AddOrganizationPanel };
