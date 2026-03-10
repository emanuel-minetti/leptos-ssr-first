import {expect, Page} from "@playwright/test";
import {test} from "./fixtures/i18n"

test.describe('layout', () => {
    const login_url = /\/login\?orig_url=.*/;
    let page: Page;
    let imprintTitle: string;
    let privacyTitle: string;
    let loginTitle: string;

    test.beforeAll(async ({i18nHelper}) => {
        imprintTitle = i18nHelper.get("en", "imprint");
        privacyTitle = i18nHelper.get("en", "privacy");
        loginTitle = i18nHelper.get("en", "login");
    });

    test.beforeEach(async ({browser}) => {
        page = await browser.newPage();
        await page.goto("/");
    });

    test.afterEach(async () => {
        await page.close();
    });

    test("has title and redirects to login page", async () => {
        await expect(page).toHaveTitle("Leptos SSR First");
        await expect(page).toHaveURL(login_url);
    });

    test("has navigation", async () => {
        const navigation = page.getByRole("navigation");
        const home_link = navigation.getByRole("link", {name: "Leptos SSR First"});
        await expect(home_link).toBeVisible();
        const lang_select = navigation.getByLabel("Language");
        await expect(lang_select).toBeVisible();
    });

    test("has footer", async () => {
        const footer = page.getByRole("contentinfo");
        const imprint_link = footer.getByRole("link", {name: imprintTitle});
        const privacy_link = footer.getByRole("link", {name: privacyTitle});
        await expect(imprint_link).toBeVisible();
        await expect(privacy_link).toBeVisible();
        const copyright_text = footer.getByText(/© 2025( - \d{2})? .+/);
        await expect(copyright_text).toBeVisible();
    });

    test("links are working", async () => {
        const navigation = page.getByRole("navigation");
        const home_link = navigation.getByRole("link", {name: "Leptos SSR First"});
        const footer = page.getByRole("contentinfo");
        const imprint_link = footer.getByRole("link", {name: imprintTitle});
        const privacy_link = footer.getByRole("link", {name: privacyTitle});

        await imprint_link.click();
        await expect(page).toHaveURL("/imprint");
        await expect(page.getByRole('heading', {name: imprintTitle})).toBeVisible();
        await privacy_link.click();
        await expect(page).toHaveURL("/privacy");
        await expect(page.getByRole('heading', {name: privacyTitle})).toBeVisible();
        await home_link.click();
        await expect(page).toHaveURL(login_url);
        await expect(page.getByRole('heading', {name: loginTitle})).toBeVisible();
    });
});
