export type Host = {
    id: string;
    ip: string;
    mac?: string;
    vendor?: string;
    hostname?: string;
    os_match?: string;
    os_accuracy?: number;
};
