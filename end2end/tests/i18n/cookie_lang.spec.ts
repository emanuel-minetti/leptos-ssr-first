import {expect, Locator, Page} from '@playwright/test';
import {test} from '../fixtures/i18n';

let page: Page;
let englishEnglish: string;
let englishGerman: string;
let germanEnglish: string;
let germanGerman: string;
let englishLogin: string;
let germanLogin: string;

let langSelect: Locator;
let heading: Locator;

test.beforeAll(async ({browser, i18nHelper}) => {
    page = await browser.newPage();
    await page.goto("/");
    englishEnglish = i18nHelper.get("en", "english");
    englishGerman = i18nHelper.get("en", "german");
    germanEnglish = i18nHelper.get("de", "english");
    germanGerman = i18nHelper.get("de", "german");
    englishLogin = i18nHelper.get("en", "login");
    germanLogin = i18nHelper.get("de", "login");

    langSelect = page.getByRole("navigation").getByLabel("Language");
    heading = page.getByRole("heading");
});
test.describe('browser lang is english', () => {
    test("browser lang is selected", async () => {
        await expect(langSelect.locator('option[selected]')).toHaveText(englishEnglish);
        await expect(heading).toHaveText(englishLogin);
    });

    test("selected lang is used and preserved", async () => {
        await langSelect.selectOption(englishGerman);
        await expect(langSelect.locator('option[selected]')).toHaveText(germanGerman);
        await expect(heading).toHaveText(germanLogin);

        await page.reload();
        await expect(langSelect.locator('option[selected]')).toHaveText(germanGerman);
        await expect(heading).toHaveText(germanLogin);
    });
});

test.describe('browser lang is german', () => {
    test.use({locale: 'de'});
    let page: Page;

    test.beforeAll(async ({browser}) => {
        page = await browser.newPage();
        await page.goto("/");
    });

    test.afterAll(async () => {
        await page.close();
    });

    test("browser lang is selected", async () => {
        await expect(langSelect.locator('option[selected]')).toHaveText(germanGerman);
        await expect(heading).toHaveText(germanLogin);
    });

    test("selected lang is used and preserved", async () => {
        await langSelect.selectOption(germanEnglish);
        await expect(langSelect.locator('option[selected]')).toHaveText(englishEnglish);
        await expect(heading).toHaveText(englishLogin);

        await page.reload();
        await expect(langSelect.locator('option[selected]')).toHaveText(englishEnglish);
        await expect(heading).toHaveText(englishLogin);
    });
});
