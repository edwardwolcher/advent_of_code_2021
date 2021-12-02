DATA_PATH = "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day2.txt"

TEST_DATA = """forward 5
down 5
forward 8
up 3
down 8
forward 2"""


class Command:
    def __init__(self, command_string) -> None:
        command_string = command_string.split()
        self.direction = command_string[0]
        self.distance = int(command_string[1])


class Position:
    def __init__(self) -> None:
        self.horizontal = 0
        self.depth = 0

    def __str__(self) -> str:
        return f"Horizontal: {self.horizontal}, Depth: {self.depth}, Product: {self.horizontal * self.depth}"

    def execute(self, command):

        if command.direction == "forward":
            self.horizontal += command.distance
        elif command.direction == "up":
            self.depth -= command.distance
        elif command.direction == "down":
            self.depth += command.distance


def main():
    position = Position()
    with open(DATA_PATH, "r") as f:
        for line in f:
            position.execute(Command(line))
    print(position)


def test():
    position = Position()
    for line in TEST_DATA.splitlines():
        position.execute(Command(line))
    print(f"Test 2_1: {position}")


if __name__ == "__main__":
    # test()
    main()
