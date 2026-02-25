import {test, expect, Page} from "@playwright/test";

test.describe('layout', () => {

  let page: Page;

  test.beforeAll(async ({ browser }) => {
    page = await browser.newPage();
    await page.goto("http://localhost:3000");
  });
  test.afterAll(async () => {
    await page.close();
  });

  test("has title and redirects to login page", async () => {
    await expect(page).toHaveTitle("Leptos SSR First");
    await expect(page).toHaveURL(/\/login\?orig_url=*/);
  });

  test("has navigation", async () => {
    const navigation = page.getByRole("navigation");
    const home_link = navigation.getByRole("link", {name: "Leptos SSR First"});
    await expect(home_link).toBeVisible();
    const lang_select = navigation.getByLabel("Language");
    await expect(lang_select).toBeVisible();
  });
});



