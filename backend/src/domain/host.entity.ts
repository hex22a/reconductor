export const expectedExistingHostId = '019d24cb-83e3-71e3-83a1-769c45462b83';
export const expectedHostIp = '192.168.0.15';

export type HostEntity = {
    id: string;
    scan_id: string;
    created_at: Date;
    ip: string;
    mac?: string;
    vendor?: string;
    hostname?: string;
    os_match?: string;
    os_accuracy?: number;
};

export type HostInsertSeed = Pick<HostEntity, 'id' | 'scan_id' | 'ip'>;
