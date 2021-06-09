
const InputScreenReaderOnly = styled.input`
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  border: 0;
`;

interface Props {
  checked: boolean;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

export default function Checkbox({ checked, onChange }: Props) {
  return (
    <CheckboxLabel>
      <InputScreenReaderOnly
        type="checkbox"
        checked={checked}
        onChange={onChange}
      />
      <span aria-hidden>{checked ? <CheckedComponent /> : <UncheckedComponent />}</span>
    </CheckboxLabel>
  );
}
