export const expectedExistingPortId = '019d24cc-105f-71cc-b5ff-bd979b90b62b';
export const expectedPort = 22;

export type PortEntity = {
    id: string;
    host_id: string;
    port: number;
    protocol?: string;
    state?: string;
    service?: string;
    product?: string;
    version?: string;
};

export type PortInsertSeed = Pick<PortEntity, 'id' | 'host_id' | 'port'>;
