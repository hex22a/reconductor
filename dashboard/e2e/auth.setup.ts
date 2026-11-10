import { test as setup } from '@playwright/test';

setup('create user and authenticate', async ({ page, request }) => {
    await request.get('/api/v1/csrf');
    const state = await request.storageState();
    const csrf = state.cookies.find((c) => c.name === 'csrf_token')?.value;

    await request.post('/api/auth/register', {
        headers: { 'x-csrf-token': csrf! },
        data: { username: 'test', password: 'password' },
    });

    await page.goto('/signin');
    await page.context().storageState({ path: 'e2e/.auth/user.json' });
});
