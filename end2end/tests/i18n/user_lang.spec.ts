import {test} from '../fixtures/database';
import {expect} from '@playwright/test';
import {LoginPage} from "../poms/loginPage";

const english_login_title = "Login";
//const german_login_title = "Anmelden"

test('german user and english browser lang', async ({page, dbHelper}) => {
    const username = await dbHelper.addTestUser('de');
    console.log(await dbHelper.query("SELECT * FROM account;"));
    const loginPage = new LoginPage(page);
    await loginPage.navigate();
    await expect(loginPage.heading).toHaveText(english_login_title);
    await loginPage.login(username, 'password');
    console.log(await dbHelper.query("SELECT * FROM account;"));
    await expect(page.getByRole('heading')).toHaveText("Startseite");
    await dbHelper.deleteTestUser(username);
});