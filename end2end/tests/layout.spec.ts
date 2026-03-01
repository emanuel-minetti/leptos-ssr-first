import {test, expect, Page} from "@playwright/test";

test.describe('layout', () => {
  const login_url = /\/login\?orig_url=.*/;
  let page: Page;

  test.beforeEach(async ({ browser }) => {
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
    const imprint_link = footer.getByRole("link", {name: "Imprint"});
    const privacy_link = footer.getByRole("link", { name: "Privacy Declaration"});
    await expect(imprint_link).toBeVisible();
    await expect(privacy_link).toBeVisible();
    const copyright_text = footer.getByText(/© 2025( - \d{2})? .+/);
    await expect(copyright_text).toBeVisible();
  });

  test("links are working", async () => {
    const navigation = page.getByRole("navigation");
    const home_link = navigation.getByRole("link", {name: "Leptos SSR First"});
    const footer = page.getByRole("contentinfo");
    const imprint_link = footer.getByRole("link", {name: "Imprint"});
    const privacy_link = footer.getByRole("link", { name: "Privacy Declaration"});

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
