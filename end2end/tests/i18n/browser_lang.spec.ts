import {expect, test} from '@playwright/test';

test.describe("browser lang", () => {
    const english_login_title = "Login";
    const german_login_title = "Anmelden"
    test.describe("sets lang to german", async () => {
        test.use({locale: 'de'});
        test("german page title is shown", async ({page}) => {
            await page.goto("/");
            await expect(page.getByRole("heading")).toHaveText(german_login_title);
        });
    });

    test.describe("sets lang not to french", async () => {
        test.use({locale: 'fr'});
        test("german page title is shown", async ({page}) => {
            await page.goto("/");
            await expect(page.getByRole("heading")).toHaveText(german_login_title);
        });
    });

    test.describe("sets lang to english on variants", async () => {
        test.use({locale: 'en-DE'});
        test("english page title is shown", async ({page}) => {
            await page.goto("/");
            await expect(page.getByRole("heading")).toHaveText(english_login_title);
        });
    });
});
