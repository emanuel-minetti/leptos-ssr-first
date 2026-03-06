import { PoolClient } from "pg";
import {test} from '../fixtures/database';
import {expect} from '@playwright/test';

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
    await page.goto("/");
    await expect(page.getByRole("heading")).toHaveText(english_login_title);

});