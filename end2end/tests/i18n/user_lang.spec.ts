import { PoolClient } from "pg";
import {test} from '../fixtures/database';
import {expect} from '@playwright/test';
import {LoginPage} from "../poms/loginPage";

let client: PoolClient;
const english_login_title = "Login";
//const german_login_title = "Anmelden"
test.beforeEach(async ({dbHelper}) => {
    client = await dbHelper.client();
    await client.query('BEGIN');
});

test.afterEach(async () => {
    await client.query('ROLLBACK');
    client.release();
});

test('german user and english browser lang', async ({page, dbHelper}) => {
    const username = await dbHelper.addTestUser('de', client);
    const loginPage = new LoginPage(page);
    await loginPage.navigate();
    await expect(loginPage.heading).toHaveText(english_login_title);
    await loginPage.login(username, 'password');
});