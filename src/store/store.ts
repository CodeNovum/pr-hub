import { createStore, createTypedHooks } from "easy-peasy";
import { ApplicationModel, IApplicationModel } from "./models/applicationModel";

interface IStoreModel {
  ApplicationModel: IApplicationModel;
}

const StoreModel: IStoreModel = {
  ApplicationModel,
};

const Store = createStore(StoreModel);

const { useStoreActions, useStoreState, useStoreDispatch, useStore } =
  createTypedHooks<IStoreModel>();

export { Store, useStore, useStoreActions, useStoreDispatch, useStoreState };
