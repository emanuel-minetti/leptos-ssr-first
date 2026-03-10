import {expect} from '@playwright/test';
import {test} from '../fixtures/i18n'

let english_login_title: string;
let german_login_title: string;

test.describe("browser lang", () => {
    test.beforeAll(async ({i18nHelper}) => {
        english_login_title = i18nHelper.get("en", "login").toString();
        german_login_title = i18nHelper.get("de", "login").toString();
    });

    test.describe("sets lang to german", async () => {
        test.use({locale: 'de'});
        test("german page title is shown", async ({page}) => {
            await page.goto("/");
            // @ts-ignore
            await expect(page.getByRole("heading")).toHaveText(german_login_title);
        });
    });

    test.describe("sets lang not to french", async () => {
        test.use({locale: 'fr'});
        test("fallback to german when unsupported language", async ({page}) => {
            await page.goto("/");
            // @ts-ignore
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
