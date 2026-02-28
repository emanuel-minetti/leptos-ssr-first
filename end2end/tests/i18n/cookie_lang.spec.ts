import {expect, Locator, Page, test} from '@playwright/test';

const german_login_title = "Anmelden";
const english_login_title = "Login";
const german_german = "Deutsch";
const english_german = "German";
const german_english = "Englisch";
const english_english = "English";

test.describe('browser lang is english', async () => {
    test_lang_setting_and_reloading();
});

test.describe('browser lang is german', async () => {
    test.use({locale: 'de'});
    test_lang_setting_and_reloading(true);
});

function test_lang_setting_and_reloading(start_with_german = false) {
    const login_title = start_with_german ? german_login_title : english_login_title;
    const other_login_title = !start_with_german ? german_login_title : english_login_title;
    const lang = start_with_german ? german_german : english_english;
    const other_lang = start_with_german ? german_english : english_german;
    const switched_lang = start_with_german ? english_english : german_german;
    let page: Page;
    let lang_select: Locator;
    let heading: Locator;

    test.beforeAll(async ({browser}) => {
        page = await browser.newPage();
        await page.goto("/");
        lang_select = page.getByRole("navigation").getByLabel("Language");
        heading = page.getByRole("heading");
    });
    test.afterAll(async () => {
        await page.close();
    });

    test("browser lang is selected", async () => {
        await expect(lang_select.locator('option[selected]')).toHaveText(lang);
        await expect(heading).toHaveText(login_title);
    });

    test("selected lang is used and preserved", async () => {
        await lang_select.selectOption(other_lang);
        await expect(lang_select.locator('option[selected]')).toHaveText(switched_lang);
        await expect(heading).toHaveText(other_login_title);
        await page.reload();
        await expect(lang_select.locator('option[selected]')).toHaveText(switched_lang);
        await expect(heading).toHaveText(other_login_title);
    });
}
