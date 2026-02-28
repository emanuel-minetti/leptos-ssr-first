import {test, expect, Page} from '@playwright/test';
const login_url = /\/login\?orig_url=*/;

test('login works the good way', async ({page}) => {
    const loginPage = await initializeLoginPage(page);
    await performLogin(loginPage, "admin", "password");
    await expect(page).toHaveURL("/");
});

test('login works the bad way (wrong username)', async ({page}) => {
    const loginPage = await initializeLoginPage(page);
    await performLogin(loginPage, "xxxxx", "password");
    await expect(page.getByText('Invalid username or password')).toBeVisible();
    await expect(page).toHaveURL(login_url);
});

test('login works the bad way (wrong password)', async ({page}) => {
    const loginPage = await initializeLoginPage(page);
    await performLogin(loginPage, "admin", "12345678");
    await expect(page.getByText('Invalid username or password')).toBeVisible();
    await expect(page).toHaveURL(login_url);
});

async function initializeLoginPage(page: Page) {
    const username_input = page.getByRole('textbox', {name: 'Username'});
    const password_input = page.getByRole('textbox', {name: 'Password'});
    const login_button = page.getByRole('button', {name: 'Login'});
    await page.goto("/");
    await expect(page).toHaveURL(login_url);
    return {username_input, password_input, login_button, page};
}

async function performLogin(loginPage: Awaited<ReturnType<typeof initializeLoginPage>>, username: string, password: string) {
    await loginPage.username_input.fill(username);
    await loginPage.password_input.fill(password);
    await loginPage.login_button.click();
}
