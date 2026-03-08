import {Page, expect} from '@playwright/test';

const LOGIN_URL_PATTERN = /\/login\?orig_url=.*/;

export class LoginPage {
    private readonly usernameInput;
    private readonly passwordInput;
    private readonly loginButton;
    heading;
    langSelect;

    constructor(private readonly page: Page) {
        this.usernameInput = page.getByRole('textbox', {name: 'Username'});
        this.passwordInput = page.getByRole('textbox', {name: 'Password'});
        this.loginButton = page.getByRole('button', {name: 'Login'});
        this.heading = page.getByRole('heading');
        this.langSelect = page.getByRole("navigation").getByLabel("Language");
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