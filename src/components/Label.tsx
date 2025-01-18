interface ILabel {
  text: string;
}

/**
 * Standardized text display
 */
const Label = (props: ILabel) => {
  return (
    <label className="label">
      <span className="label-text">{props.text}</span>
    </label>
  );
};

export { Label };
