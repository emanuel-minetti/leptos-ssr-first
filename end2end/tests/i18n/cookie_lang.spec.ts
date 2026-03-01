import {expect, Page, test} from '@playwright/test';

test.describe('browser lang is english', () => {
    let page: Page;

    test.beforeAll(async ({browser}) => {
        page = await browser.newPage();
        await page.goto("/");
    });

    test.afterAll(async () => {
        await page.close();
    });

    test("browser lang is selected", async () => {
        const langSelect = page.getByRole("navigation").getByLabel("Language");
        const heading = page.getByRole("heading");
        await expect(langSelect.locator('option[selected]')).toHaveText("English");
        await expect(heading).toHaveText("Login");
    });

    test("selected lang is used and preserved", async () => {
        const langSelect = page.getByRole("navigation").getByLabel("Language");
        const heading = page.getByRole("heading");

        await langSelect.selectOption("German");
        await expect(langSelect.locator('option[selected]')).toHaveText("Deutsch");
        await expect(heading).toHaveText("Anmelden");

        await page.reload();
        await expect(langSelect.locator('option[selected]')).toHaveText("Deutsch");
        await expect(heading).toHaveText("Anmelden");
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
        const langSelect = page.getByRole("navigation").getByLabel("Language");
        const heading = page.getByRole("heading");
        await expect(langSelect.locator('option[selected]')).toHaveText("Deutsch");
        await expect(heading).toHaveText("Anmelden");
    });

    test("selected lang is used and preserved", async () => {
        const langSelect = page.getByRole("navigation").getByLabel("Language");
        const heading = page.getByRole("heading");

        await langSelect.selectOption("Englisch");
        await expect(langSelect.locator('option[selected]')).toHaveText("English");
        await expect(heading).toHaveText("Login");

        await page.reload();
        await expect(langSelect.locator('option[selected]')).toHaveText("English");
        await expect(heading).toHaveText("Login");
    });
});
