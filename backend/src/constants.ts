const DEFAULT_DASHBOARD_URL = 'http://localhost:5173';
export const DASHBOARD_URL: string = (process.env.NODE_ENV === 'production' ? process.env.DASHBOARD_URL : DEFAULT_DASHBOARD_URL) || DEFAULT_DASHBOARD_URL;
