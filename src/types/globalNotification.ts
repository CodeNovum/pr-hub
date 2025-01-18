import { ToastType } from "../components/Toast";

interface IGlobalNotification {
  type: ToastType;
  message: string;
}

export type { IGlobalNotification };
