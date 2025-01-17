import { useEffect, useState } from "react";

/**
 * Hook to that keeps track of whether the dark mode is
 * enabled or not
 *
 * @returns {boolean} Whether the dark mode is enabled or not
 */
const useIsDarkModeEnabled = (): boolean => {
  const [isDarkModeEnabled, setIsDarkModeEnabled] = useState<boolean>(false);

  useEffect(() => {
    const htmlElement = document.documentElement;
    const checkIfDarkModeIsEnabled = () => {
      if (htmlElement.hasAttribute("data-theme")) {
        const theme = htmlElement.getAttribute("data-theme");
        setIsDarkModeEnabled(theme === "dark");
        return;
      }
      setIsDarkModeEnabled(
        window.matchMedia("(prefers-color-scheme: dark)").matches,
      );
    };
    htmlElement.addEventListener("input", checkIfDarkModeIsEnabled);
    checkIfDarkModeIsEnabled();
    return () =>
      htmlElement.removeEventListener("input", checkIfDarkModeIsEnabled);
  }, []);

  return isDarkModeEnabled;
};

export { useIsDarkModeEnabled };
