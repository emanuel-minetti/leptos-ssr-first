import {test, expect} from '@playwright/test';
import {LoginPage} from "./poms/loginPage";

const VALID_USERNAME = "admin";
const VALID_PASSWORD = "password";

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
