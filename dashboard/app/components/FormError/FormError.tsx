import { CrossIcon } from '~/ui/vector/cross';

type FormErrorProps = {
  error: string;
  onClose: () => void;
};

export function FormError({ error, onClose }: FormErrorProps) {
  return (
    <div className="relative rounded-xl border border-red-500 bg-red-100 p-4 text-red-500">
      {error}
      <button onClick={onClose} className="absolute top-4 right-1">
        <CrossIcon width={24} height={24} />
      </button>
    </div>
  );
}
