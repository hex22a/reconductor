import type { ReactNode } from 'react';

export type Column<T> = {
  key: keyof T;
  label: string;
  render?: (value: T[keyof T], id: string) => ReactNode;
};

export type TableProps<T> = {
  columns: Array<Column<T>>;
  edges: ReadonlyArray<{ node: T }>;
};

export function Table<T extends { id: string } & Record<string, unknown>>({
  columns,
  edges,
}: TableProps<T>) {
  return (
    <table className="my-2 w-full">
      <thead className="py-2">
        <tr>
          {columns.map((column) => (
            <th className="p-1 text-left">{column.label}</th>
          ))}
        </tr>
      </thead>
      <tbody>
        {edges.map((edge) => (
          <tr>
            {columns.map((column) => (
              <td key={column.key.toString()} className="p-1">
                {column.render
                  ? column.render(edge.node[column.key], edge.node.id)
                  : String(edge.node[column.key] ?? '---')}
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  );
}
