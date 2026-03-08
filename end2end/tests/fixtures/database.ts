import {test as base} from '@playwright/test';
import {Client} from 'pg'

export class DatabaseHelper {
    private readonly client: Client;
    private readonly workerId: number;

    constructor(id: number) {
        this.workerId = id;
        this.client = new Client({
            user: 'lsf',
            password: 'lsf',
            host: 'localhost',
            database: 'lsf_test',
            port: 5432,
        });
    }

    async connect() {
        await this.client.connect();
    }

    async query(sql: string, params: any[] = []) {
        const result = await this.client.query(sql, params);
        return result.rows;
    }

    async addTestUser(lang: string) {
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