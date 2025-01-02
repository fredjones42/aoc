# Advent of Code 2015 Day 02 - Python Solution Stub


def main():
    try:
        with open("../input.txt", "r") as f:
            data = [[int(n) for n in s.split("x")] for s in f.read().splitlines()]
            total = sum(calc_area(x) for x in data)
            print(f"The elves should order {total} square feet of wrapping paper")
            total = sum(calc_length(x) for x in data)
            print(f"The elves should order {total} feet of ribbon")
    except FileNotFoundError:
        print("Input file not found")


def calc_area(nums):
    L, w, h = nums
    lw = L * w
    lh = L * h
    wh = w * h
    minimum = min(lw, lh, wh)
    return 2 * lw + 2 * lh + 2 * wh + minimum


def calc_length(nums):
    L, w, h = nums
    lw = 2 * L + 2 * w
    lh = 2 * L + 2 * h
    wh = 2 * w + 2 * h
    minimum = min(lw, lh, wh)
    volume = L * w * h
    return minimum + volume


if __name__ == "__main__":
    main()
