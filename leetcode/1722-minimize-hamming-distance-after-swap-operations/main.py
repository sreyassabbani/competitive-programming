def minimumHammingDistance(
    source: list[int], target: list[int], allowedSwaps: list[list[int]]
) -> int:
    idxs = set(i for swap in allowedSwaps for i in swap)
    n = len(source)

    req: dict[int, int] = dict()

    for i in range(n):
        s = source[i]
        t = target[i]
        if s != t:
            req[t] = i

    print(req)
    swaps = 0
    working = source[:]
    for i, s in enumerate(source):
        needs_to_be_at = req.get(s)
        if (
            (needs_to_be_at is not None)
            and (needs_to_be_at != i)
            and (i in idxs and needs_to_be_at in idxs)
        ):
            tmp = working[needs_to_be_at]
            working[needs_to_be_at] = working[i]
            working[i] = tmp
            swaps += 1

    return swaps


tests = [  # pyright: ignore[reportUnknownVariableType]
    ([1, 2, 3, 4], [2, 1, 4, 5], [[0, 1], [2, 3]]),
    ([1, 2, 3, 4], [1, 3, 2, 4], []),
    ([5, 1, 2, 4, 3], [1, 5, 4, 2, 3], [[0, 4], [4, 2], [1, 3], [1, 4]]),
]

print("recvd: ", [minimumHammingDistance(a, b, c) for a, b, c in tests])
print("expct: ", [1, 2, 0])
