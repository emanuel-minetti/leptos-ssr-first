import {test as dbTest} from '../fixtures/database';
import {test as lpTest} from '../fixtures/loginPage';
import {expect, mergeTests} from '@playwright/test';

const test = mergeTests(dbTest, lpTest);

let english_login_title: string;
let german_login_title: string;
let english_home_title: string;
let german_home_title: string;

test.beforeAll(async ({i18nHelper}) => {
    english_login_title = i18nHelper.get("en", "login");
    german_login_title = i18nHelper.get("de", "login");
    english_home_title = i18nHelper.get("en", "homePageTitle");
    german_home_title = i18nHelper.get("de", "homePageTitle");
});

test.describe("browser lang is english", async () => {
    test('user lang is german', async ({page, dbHelper, loginPage}) => {
        const username = await dbHelper.addTestUser('de');
        await loginPage.navigate();
        await expect(loginPage.heading).toHaveText(english_login_title);
        await loginPage.login(username);
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
        await loginPage.login(username);
        await expect(page.getByRole('heading')).toHaveText(english_home_title);
        await dbHelper.deleteTestUser(username);
    });
});
