import {expect, Locator, Page} from '@playwright/test';
import {test as base, I18n} from "./i18n";

const LOGIN_URL_PATTERN = /\/login\?orig_url=.*/;

class LoginPage {
    private readonly lang: String;
    private readonly i18nHelper: I18n;
    private readonly usernameInput: Locator;
    private readonly passwordInput;
    private readonly loginButton: Locator;
    readonly heading: Locator;
    readonly langSelect: Locator;

    constructor(private readonly page: Page, lang: String, i18nHelper: I18n) {
        this.lang = lang;
        this.i18nHelper = i18nHelper;
        const usernameText = i18nHelper.get(lang, "username")!;
        const passwordText = i18nHelper.get(lang, "password")!;
        const loginText = i18nHelper.get(lang, "login")!;
        this.usernameInput = page.getByRole('textbox', {name: usernameText});
        this.passwordInput = page.getByRole('textbox', {name: passwordText});
        this.loginButton = page.getByRole('button', {name: loginText});
        this.heading = page.getByRole('heading');
        this.langSelect = page.getByRole("navigation").getByLabel("Language");
    }

    async navigate() {
        await this.page.goto("/");
        await expect(this.page).toHaveURL(LOGIN_URL_PATTERN);
    }

    // noinspection JSUnusedGlobalSymbols
    async login(username: string, password = 'password') {
        await this.usernameInput.fill(username);
        await this.passwordInput.fill(password);
        await this.loginButton.click();
    }

    async expectInvalidCredentialsError() {
        const text = this.i18nHelper.get(this.lang, "invalidCredentials")
        await expect(this.page.getByText(text)).toBeVisible();
        await expect(this.page).toHaveURL(LOGIN_URL_PATTERN);
    }
}

// noinspection JSVoidFunctionReturnValueUsed,JSUnusedGlobalSymbols
export const test = base.extend<{loginPage: LoginPage;}>({
    loginPage: async ({ page, i18nHelper }, use) => {
        await page.goto("/");
        const lang = (await page.evaluate(() => navigator.language)).substring(0, 2);
        await use(new LoginPage(page, lang!, i18nHelper));
    },
});