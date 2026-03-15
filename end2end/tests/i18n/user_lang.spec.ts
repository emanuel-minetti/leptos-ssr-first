import {test as dbTest} from '../fixtures/database';
import {test as lpTest} from '../fixtures/loginPage';
import {expect, mergeTests} from '@playwright/test';

const test = mergeTests(dbTest, lpTest);

let englishLoginTitle: string;
let germanLoginTitle: string;
let englishHomeTitle: string;
let germanHomeTitle: string;
let englishGerman: string;
let germanEnglish: string;

test.beforeAll(async ({i18nHelper}) => {
    englishLoginTitle = i18nHelper.get("en", "login");
    germanLoginTitle = i18nHelper.get("de", "login");
    englishHomeTitle = i18nHelper.get("en", "homePageTitle");
    germanHomeTitle = i18nHelper.get("de", "homePageTitle");
    englishGerman = i18nHelper.get("en", "german");
    germanEnglish = i18nHelper.get("de", "english");
});

test.describe("browser lang is english", async () => {
    test('user lang is german', async ({page, dbHelper, loginPage}) => {
        const username = await dbHelper.addTestUser('de');
        await loginPage.navigate();
        await expect(loginPage.heading).toHaveText(englishLoginTitle);
        await loginPage.login(username);
        await expect(page.getByRole('heading')).toHaveText(germanHomeTitle);
        await dbHelper.deleteTestUser(username);
    });

    test('setting the lang as user is persisted in db and shown on next login',
        async ({page, dbHelper, loginPage, browser}) => {
            const username = await dbHelper.addTestUser('en');
            await loginPage.navigate();
            await loginPage.login(username);
            await page.waitForURL("/");
            await loginPage.setLang(englishGerman);
            await expect(page.getByRole('heading')).toHaveText(germanHomeTitle);
            await page.waitForTimeout(500);
            expect(await dbHelper.getUserLang(username)).toBe("de");
            await browser.newContext();
            await page.evaluate(() => localStorage.removeItem('lang'));
            await loginPage.navigate();
            await expect(loginPage.heading).toHaveText(englishLoginTitle);
            await loginPage.login(username);
            await expect(page.getByRole('heading')).toHaveText(germanHomeTitle);
            // @ts-ignore
            await browser.contexts().pop().close();
            await dbHelper.deleteTestUser(username);
        });
});

test.describe("browser lang is german", async () => {
    test.use({locale: 'de'});
    test('user lang is english', async ({page, dbHelper, loginPage}) => {
        const username = await dbHelper.addTestUser('en');
        await loginPage.navigate();
        await expect(loginPage.heading).toHaveText(germanLoginTitle);
        await loginPage.login(username);
        await expect(page.getByRole('heading')).toHaveText(englishHomeTitle);
        await dbHelper.deleteTestUser(username);
    });

    test('setting the lang as user is persisted in db and shown on next login',
        async ({page, dbHelper, loginPage, browser}) => {
            const username = await dbHelper.addTestUser('de');
            await loginPage.navigate();
            await loginPage.login(username);
            await page.waitForURL("/");
            await loginPage.setLang(germanEnglish);
            await expect(page.getByRole('heading')).toHaveText(englishHomeTitle);
            expect(await dbHelper.getUserLang(username)).toBe("en");
            await browser.newContext();
            await page.evaluate(() => localStorage.removeItem('lang'));
            await loginPage.navigate();
            await expect(loginPage.heading).toHaveText(germanLoginTitle);
            await loginPage.login(username);
            await expect(page.getByRole('heading')).toHaveText(englishHomeTitle);
            // @ts-ignore
            await browser.contexts().pop().close();
            await dbHelper.deleteTestUser(username);
        });
});
