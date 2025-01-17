import { BusySpinner } from "./BusySpinner";

/**
 * Display the styled busy spinner as overlay
 */
const BusySpinnerOverlay = () => {
  return (
    <div className="absolute top-0 left-0 z-40 flex h-full w-full items-center justify-center bg-gray-500 bg-opacity-75">
      <BusySpinner size={"Medium"} />
    </div>
  );
};

export { BusySpinnerOverlay };
