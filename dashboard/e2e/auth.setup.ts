import { test as setup, expect } from '@playwright/test';

const API = 'http://localhost:3000/api/v1';
const user = { username: 'test@example.com', password: 'password' };

setup('register and authenticate', async ({ page, request }) => {
    const csrfRes = await request.get(`${API}/csrf`);
    expect(csrfRes.ok()).toBeTruthy();
    const { csrf_token } = await csrfRes.json();

    const reg = await request.post(`${API}/register`, {
        headers: { 'X-CSRF-Token': csrf_token },
        data: user,
    });
    expect(reg.ok()).toBeTruthy();

    await page.goto('/signin');
    await page.getByLabel('Username:').fill(user.username);
    await page.getByLabel('Password').fill(user.password);
    await page.getByRole('button', { name: 'Sign in' }).click();
    await expect(page).toHaveURL('/projects');

    await page.context().storageState({ path: 'e2e/.auth/user.json' });
});
