import {expect} from '@playwright/test';
import {test} from "./fixtures/loginPage";

const VALID_USERNAME = "admin";

test('login works the good way', async ({page, loginPage}) => {
    await loginPage.navigate();
    await loginPage.login(VALID_USERNAME);
    await expect(page).toHaveURL("/");
});

test('login works the bad way (wrong username)', async ({loginPage}) => {
    await loginPage.navigate();
    await loginPage.login("xxxxx");
    await loginPage.expectInvalidCredentialsError();
});

test('login works the bad way (wrong password)', async ({loginPage}) => {
    await loginPage.navigate();
    await loginPage.login(VALID_USERNAME, "12345678");
    await loginPage.expectInvalidCredentialsError();
});
