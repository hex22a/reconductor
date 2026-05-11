import { CrossIcon } from '~/ui/vector/cross';

type FormErrorProps = {
  error: string;
  onClose: () => void;
};

export function FormError({ error, onClose }: FormErrorProps) {
  return (
    <div className="relative rounded-xl border border-red-300 p-2 pr-5 text-red-300">
      {error}
      <button onClick={onClose} className="absolute top-4 right-1 cursor-pointer">
        <CrossIcon className="fill-red-300 opacity-60 hover:opacity-80" width={24} height={24} />
      </button>
    </div>
  );
}
