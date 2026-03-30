import { z } from 'zod';
import {
    Z_SCAN_PROJECT_ID_ERROR_MESSAGE,
    Z_SCAN_SCHEMA_ERROR_MESSAGE,
    Z_SCAN_SHCEDULE_ERROR_MESSAGE,
    Z_SCAN_TARGET_ERROR_MESSAGE,
} from '../constants';

const cronRegex =
    /^(\*|([0-5]?\d)) (\*|([01]?\d|2[0-3])) (\*|([12]?\d|3[01])) (\*|(1[0-2]|[1-9])) (\*|([0-7]))$/;

export const scanSchema = z.object(
    {
        target: z.union([z.ipv4(), z.ipv6(), z.cidrv4(), z.cidrv6()], {
            error: Z_SCAN_TARGET_ERROR_MESSAGE,
        }),
        projectId: z.uuidv7({ error: Z_SCAN_PROJECT_ID_ERROR_MESSAGE }),
        schedule: z.string().regex(cronRegex, { error: Z_SCAN_SHCEDULE_ERROR_MESSAGE }).optional(),
    },
    { error: Z_SCAN_SCHEMA_ERROR_MESSAGE },
);
