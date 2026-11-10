import { test, expect } from '@playwright/test';

test('projects table loads', async ({ page }) => {
    await page.goto('/projects');
    await expect(page.getByRole('table')).toBeVisible();
    await expect(page.getByRole('row')).not.toHaveCount(1);
});
