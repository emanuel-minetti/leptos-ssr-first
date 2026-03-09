import {test as base} from '@playwright/test';
import {Client} from 'pg'
import * as fs from "node:fs/promises";


class DatabaseHelper {
    private client: Client | undefined;
    private readonly workerId: number;

    constructor(id: number) {
        this.workerId = id;

    }

    async connect() {
        const configFile = await fs.readFile('../config/configuration.test.json');
        const config: any = JSON.parse(configFile.toString());
        const dbConfig: any = config.database;
        this.client = new Client({
            user: dbConfig.username,
            password: dbConfig.password,
            host: dbConfig.host,
            database: dbConfig.database_name,
            port: dbConfig.port,
        });
        await this.client.connect();
    }

    async query(sql: string, params: any[] = []) {
        if (!this.client) {
            throw new Error("Call connect() before using the client.");
        }
        const result = await this.client.query(sql, params);
        return result.rows;
    }

    async addTestUser(lang: string) {
        if (!this.client) {
            throw new Error("Call connect() before using the client.");
        }
        const username = lang + "_testuser_" + this.workerId;
        // 'password' hashed
        const hash = "$2a$12$2W3AcX2RnI3ZJSwrvWbar.x6FL.nK63niONl.d.mv39bTG5Ru/E9G";
        const name = "Test User";
        const query = "INSERT\n\t" +
            "INTO account (username, pw_hash, name, preferred_language)\n\t" +
            "VALUES ($1, $2, $3, $4)";
        await this.client.query(query, [username, hash, name, lang]);
        return username;
    }

    async deleteTestUser(username: string) {
        if (!this.client) {
            throw new Error("Call connect() before using the client.");
        }
        const query = "DELETE FROM account WHERE username = $1";
        await this.client.query(query, [username]);
    }
    async disconnect() {
        if (this.client) {
            await this.client.end();
        }
    }
}

// noinspection JSVoidFunctionReturnValueUsed
export const test = base.extend<{}, {dbHelper: DatabaseHelper;}>({
    dbHelper: [
        async ({}, use, workerInfo) => {
            const dbHelper = new DatabaseHelper(workerInfo.workerIndex);
            await dbHelper.connect();
            await use(dbHelper);
            await dbHelper.disconnect();
        }, {scope: 'worker'}],
});