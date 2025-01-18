import { MoonIcon, SunIcon } from "@heroicons/react/24/solid";
import { useIsDarkModeEnabled } from "../hooks/useIsDarkModeEnabled";

/**
 * Button that toggles between light and dark mode
 * based on the currently active mode
 */
const DarkModeToggle = () => {
  const isDarkModeEnabled = useIsDarkModeEnabled();

  const handleThemeSwitch = (enableDarkMode: boolean) => {
    const htmlElement = document.documentElement;
    if (enableDarkMode) {
      htmlElement.setAttribute("data-theme", "dark");
    } else {
      htmlElement.setAttribute("data-theme", "light");
    }
  };

  return (
    <label className="swap btn-ghost swap-rotate btn h-12 w-12 animate-none">
      <input
        type="checkbox"
        checked={isDarkModeEnabled}
        onChange={(event) => {
          const checked = event.target.checked;
          handleThemeSwitch(checked);
        }}
      />
      <SunIcon className="swap-off h-5 w-5" />
      <MoonIcon className="current swap-on h-5 w-5" />
    </label>
  );
};

export { DarkModeToggle };
