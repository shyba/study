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

if __name__ == '__main__':
    asyncio.run(main())