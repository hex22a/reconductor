import { defineConfig } from '@playwright/test';

export default defineConfig({
    testDir: './e2e',
    fullyParallel: true,
    retries: process.env.CI ? 2 : 0,
    reporter: 'list',
    use: {
        baseURL: 'http://localhost:4173',
        trace: 'on-first-retry',
    },
    webServer: [
        {
            command: 'cargo run',
            cwd: '../server',
            url: 'http://localhost:3000/api/v1/health',
            reuseExistingServer: !process.env.CI,
            timeout: 120_000,
        },
        {
            command: 'pnpm build && pnpm preview -- --port 4173',
            url: 'http://localhost:4173',
            reuseExistingServer: !process.env.CI,
        },
    ],
});
