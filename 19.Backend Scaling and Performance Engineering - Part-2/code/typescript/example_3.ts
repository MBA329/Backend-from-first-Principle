import Redis from 'ioredis';

const rdb = new Redis();

interface EmailJob {
    to: string;
    subject: string;
    template: string;
}

export async function enqueueEmail(job: EmailJob) {
    await rdb.lpush("queue:emails", JSON.stringify(job));
}

export async function emailWorker() {
    while (true) {
        const result = await rdb.brpop("queue:emails", 0);
        if (result) {
            const job: EmailJob = JSON.parse(result[1]);
            // sendEmail(job) takes 300ms, but no user is waiting
        }
    }
}
