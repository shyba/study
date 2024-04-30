import time
import asyncio
from typing import List, Callable

async def sleep_and_do(sleep_seconds: int):
    await asyncio.sleep(sleep_seconds / 1000.)
    return sleep_seconds

async def sort(vals: List) -> List:
    return [await x for x in asyncio.as_completed([sleep_and_do(val) for val in vals])]

async def sort_broken(vals: List) -> List:
    return [x for x in await asyncio.gather(*[sleep_and_do(val) for val in vals])]

async def main():
    original = [10, 0, 2, 6]
    print(f"original: {original}")
    print(f"sorted: {await sort(original)}")
    print(f"sorted: {await sort_broken(original)}")

async def sleep_furiously():
    start = time.time()
    loop = asyncio.get_event_loop()
    original_clock = loop.time
    drift = 0
    loop.time = lambda: original_clock() + drift
    original_data = [10, 0, 2, 6, 100_000_000]
    pending = asyncio.ensure_future(sort(original_data))
    while not pending.done():
        drift += 1
        await asyncio.sleep(0)
    print(await pending)
    print("Took %.2f seconds in this universe." % (time.time()-start))

if __name__ == '__main__':
    asyncio.run(main())
    asyncio.run(sleep_furiously())