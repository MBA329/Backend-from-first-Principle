
balance = 100
lock = asyncio.Lock()

async function withdraw(amount: number):
    global balance
    async with lock:                     # Only one coroutine at a time
        if balance >= amount:
            await process_withdrawal(amount)
            balance -= amount

# Now safe ,  second withdraw waits for the lock
await asyncio.gather(
    withdraw(100),
    withdraw(100),
)
print(balance)  # 0, not -100
