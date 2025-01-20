import { IGlobalNotification } from "../../types/globalNotification";
import { Action, action } from "easy-peasy";

interface IApplicationModel {
  globalNotification?: IGlobalNotification;
  updateGlobalNotificationMessage: Action<
    IApplicationModel,
    IGlobalNotification | undefined
  >;
  isSideNavCollapsed: boolean;
  updateIsSideNavCollapsed: Action<IApplicationModel, boolean>;
}

const ApplicationModel: IApplicationModel = {
  updateGlobalNotificationMessage: action((state, payload) => {
    state.globalNotification = payload;
  }),
  isSideNavCollapsed: true,
  updateIsSideNavCollapsed: action((state, payload) => {
    state.isSideNavCollapsed = payload;
  }),
};

export type { IApplicationModel };
export { ApplicationModel };
