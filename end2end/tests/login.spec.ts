import {test, expect, Page} from '@playwright/test';

const LOGIN_URL_PATTERN = /\/login\?orig_url=.*/;
const VALID_USERNAME = "admin";
const VALID_PASSWORD = "password";

class LoginPage {
    private readonly usernameInput;
    private readonly passwordInput;
    private readonly loginButton;

    constructor(private readonly page: Page) {
        this.usernameInput = page.getByRole('textbox', {name: 'Username'});
        this.passwordInput = page.getByRole('textbox', {name: 'Password'});
        this.loginButton = page.getByRole('button', {name: 'Login'});
    }

    async navigate() {
        await this.page.goto("/");
        await expect(this.page).toHaveURL(LOGIN_URL_PATTERN);
    }

    async login(username: string, password: string) {
        await this.usernameInput.fill(username);
        await this.passwordInput.fill(password);
        await this.loginButton.click();
    }

    async expectInvalidCredentialsError() {
        await expect(this.page.getByText('Invalid username or password')).toBeVisible();
        await expect(this.page).toHaveURL(LOGIN_URL_PATTERN);
    }
}

test('login works the good way', async ({page}) => {
    const loginPage = new LoginPage(page);
    await loginPage.navigate();
    await loginPage.login(VALID_USERNAME, VALID_PASSWORD);
    await expect(page).toHaveURL("/");
});

test('login works the bad way (wrong username)', async ({page}) => {
    const loginPage = new LoginPage(page);
    await loginPage.navigate();
    await loginPage.login("xxxxx", VALID_PASSWORD);
    await loginPage.expectInvalidCredentialsError();
});

test('login works the bad way (wrong password)', async ({page}) => {
    const loginPage = new LoginPage(page);
    await loginPage.navigate();
    await loginPage.login(VALID_USERNAME, "12345678");
    await loginPage.expectInvalidCredentialsError();
});
