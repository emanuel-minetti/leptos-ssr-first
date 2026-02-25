import {test, expect, Page, Locator} from "@playwright/test";

test.describe('layout', () => {

  let page: Page;
  let navigation: Locator;
  let home_link: Locator;
  let footer: Locator;
  let imprint_link: Locator;
  let privacy_link: Locator;
  const login_url = /\/login\?orig_url=*/;

  test.beforeAll(async ({ browser }) => {
    page = await browser.newPage();
    navigation = page.getByRole("navigation");
    home_link = navigation.getByRole("link", {name: "Leptos SSR First"});
    footer = page.getByRole("contentinfo");
    imprint_link = footer.getByRole("link", {name: "Imprint"});
    privacy_link = footer.getByRole("link", { name: "Privacy Declaration"});
    await page.goto("/");
  });
  test.afterAll(async () => {
    await page.close();
  });

  test("has title and redirects to login page", async () => {
    await expect(page).toHaveTitle("Leptos SSR First");
    await expect(page).toHaveURL(login_url);
  });

  test("has navigation", async () => {
    await expect(home_link).toBeVisible();
    const lang_select = navigation.getByLabel("Language");
    await expect(lang_select).toBeVisible();
  });

  test("has footer", async () => {
    await expect(imprint_link).toBeVisible();
    await expect(privacy_link).toBeVisible();
    const copyright_text = footer.getByText(/© 2025( - \d{2})? .+/);
    await expect(copyright_text).toBeVisible();
  });

  test("links are working", async () => {
    await imprint_link.click();
    await expect(page).toHaveURL("/imprint");
    await expect(page.getByRole('heading', { name: 'Imprint' })).toBeVisible();
    await privacy_link.click();
    await expect(page).toHaveURL("/privacy");
    await expect(page.getByRole('heading', { name: 'Privacy Declaration' })).toBeVisible();
    await home_link.click();
    await expect(page).toHaveURL(login_url);
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });
});
