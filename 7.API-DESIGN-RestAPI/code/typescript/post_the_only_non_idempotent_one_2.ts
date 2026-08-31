import { Request, Response } from 'express';

export async function createPayment(req: Request, res: Response) {
    const key = req.get('Idempotency-Key'); // client-generated UUID
    if (!key) {
        return res.status(400).json({ error: "Idempotency-Key header is required" });
    }

    // already processed this exact request? return the SAME result, don't re-charge
    const prior = await idemStore.get(key);
    if (prior) {
        return res.status(prior.status).json(prior.body);
    }

    const { amount } = req.body;

    const payment = await charge(amount); // the real, non-idempotent side effect
    await idemStore.save(key, 201, payment); // remember it, keyed by the idempotency key
    
    return res.status(201).json(payment);
}

// Mock store/functions for compilation
const idemStore = {
    get: async (key: string): Promise<any> => null,
    save: async (key: string, status: number, data: any): Promise<void> => {}
};
const charge = async (amount: number): Promise<any> => ({ amount, id: "pay_123" });
