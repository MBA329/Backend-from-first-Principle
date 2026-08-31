import Redis from 'ioredis';

const rdb = new Redis({ host: 'redis', port: 6379 });

export async function storeSession(sessionID: string, userID: string) {
    await rdb.set("session:" + sessionID, userID, 'EX', 24 * 60 * 60);
}

export async function getSession(sessionID: string) {
    return await rdb.get("session:" + sessionID);
}
