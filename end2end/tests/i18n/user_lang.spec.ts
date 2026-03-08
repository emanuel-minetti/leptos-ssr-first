import {test as dbTest} from '../fixtures/database';
import {test as lpTest} from '../fixtures/loginPage';
import {expect, mergeTests} from '@playwright/test';

const test = mergeTests(dbTest, lpTest);

const english_login_title = "Login";
const german_login_title = "Anmelden";
const english_home_title = "Home Page";
const german_home_title = "Startseite"

test.describe("browser lang is english", async () => {
    test('user lang is german', async ({page, dbHelper, loginPage}) => {
        const username = await dbHelper.addTestUser('de');
        await loginPage.navigate();
        await expect(loginPage.heading).toHaveText(english_login_title);
        await loginPage.login(username, 'password');
        await expect(page.getByRole('heading')).toHaveText(german_home_title);
        await dbHelper.deleteTestUser(username);
    });
});

test.describe("browser lang is german", async () => {
    test.use({locale: 'de'});
    test('user lang is english', async ({page, dbHelper, loginPage}) => {
        const username = await dbHelper.addTestUser('en');
        await loginPage.navigate();
        await expect(loginPage.heading).toHaveText(german_login_title);
        await loginPage.login(username, 'password');
        await expect(page.getByRole('heading')).toHaveText(english_home_title);
        await dbHelper.deleteTestUser(username);
    });
});
