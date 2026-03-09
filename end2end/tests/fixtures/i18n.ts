import * as fs from "node:fs/promises";
import {test as base} from '@playwright/test';


export class I18n {
    private readonly path = "../locales/"
    private englishTranslations: Object | undefined;
    private germanTranslations: Object | undefined;

    async init()  {
        if (!this.englishTranslations) {
            const file = await fs.readFile(this.path + "en.json");
            this.englishTranslations = JSON.parse(file.toString());
        }
        if (!this.germanTranslations) {
            const file = await fs.readFile(this.path + "de.json");
            this.germanTranslations = JSON.parse(file.toString());
        }
    }

    get(lang: String, key: String) {
        if (!(this.englishTranslations && this.germanTranslations)) {
            throw new Error("Run init() before using get()!");
        }
        if (lang == "en") {
            // @ts-ignore
            return this.englishTranslations[key.toString()];
        } else {
            // @ts-ignore
            return this.germanTranslations[key.toString()];
        }
    }
}

// noinspection JSVoidFunctionReturnValueUsed
export const test = base.extend<{}, {i18nHelper: I18n}>({
    i18nHelper: [
        async ({}, use) => {
            const i18nHelper = new I18n();
            await i18nHelper.init();
            await use(i18nHelper);
        }, {scope: 'worker'}
    ],
});