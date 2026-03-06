import {test as base} from '@playwright/test';
import {Pool, PoolClient} from 'pg'

export class DatabaseHelper {
    private pool: Pool | undefined;
    private workerId: number;

    constructor(id: number) {
        this.workerId = id;
    }

    async connect() {
        this.pool = new Pool({
            user: 'lsf',
            password: 'lsf',
            host: 'localhost',
            database: 'lsf_test',
            port: 5432,
        });
    }

    async client() {
        if (!this.pool) {
            throw new Error('Database not connected. Call connect() first.');
        }
        return this.pool.connect();
    }

    async query(sql: string, params: any[] = []) {
        const client = await this.client();
        try {
            const result = await client.query(sql, params);
            return result.rows;
        } finally {
            client.release();
        }
    }

    async addTestUser(lang: string, client: PoolClient) {
        const username = lang + "_testuser_" + this.workerId;
        // 'password' hashed
        const hash = "$2a$12$2W3AcX2RnI3ZJSwrvWbar.x6FL.nK63niONl.d.mv39bTG5Ru/E9G";
        const name = "Test User";
        const query = "INSERT\n\t" +
            "INTO account (username, pw_hash, name, preferred_language)\n\t" +
            "VALUES ($1, $2, $3, $4)";
        await client.query(query, [username, hash, name, lang]);
        return username;
    }
    async disconnect() {
        if (this.pool) {
            await this.pool.end();
        }
    }
}

export const test = base.extend<{}, {dbHelper: DatabaseHelper;}>({
    dbHelper: [
        async ({}, use, workerInfo) => {
            const dbHelper = new DatabaseHelper(workerInfo.workerIndex);
            await dbHelper.connect();
            await use(dbHelper);
            await dbHelper.disconnect();
        }, {scope: 'worker'}],
});