import random

SIZE = 5

def random_new_world():
    new_world = [[random.randrange(0, 2, 1) for i in range(SIZE)] for j in range(SIZE)]
    return new_world

def world_from_file():
    fhand = open('lib/' + input('Enter file name: '))
    new_world = list()
    for line in fhand:
        line = line.strip()
        row = list()
        for number in line:
            row.append(int(number))
        new_world.append(row)
    SIZE = len(new_world)
    return new_world

def print_world(world):
    for i in range(SIZE):
        print(world[i])
    print()

def census(world):
    count = 0
    for i in range(len(world)):
        for j in range(len(world)):
            if not world[i][j]:
                continue
            count += 1
    return count

def count_neighbors(row, column, world):
    count = 0

    if row > 0 and column > 0: # top left
        count += world[row - 1][column - 1]

    if row > 0: # left
        count += world[row - 1][column]

    if row > 0 and column < SIZE - 1: # bottom left
        count += world [row - 1][column + 1]

    if column > 0: #top
        count += world[row][column - 1]

    if column < SIZE - 1: # bottom
        count += world[row][column + 1]

    if row < SIZE - 1 and column > 0: # top right
        count += world[row + 1][column - 1]

    if row < SIZE - 1: # right
        count += world[row + 1][column]

    if row < SIZE - 1 and column < SIZE - 1:
        count += world[row + 1][column + 1]
        
    return count

def generate(world):
    new_world = [[0 for i in range(SIZE)] for j in range(SIZE)]
    for i in range(SIZE):
        for j in range(SIZE):
            neighbor_count = count_neighbors(i, j, world)
            if not world[i][j]:
                if neighbor_count >= 3:
                    new_world[i][j] = 1
                continue
            if neighbor_count < 2 or neighbor_count > 3:
                new_world[i][j] = 0
    return new_world

if __name__ == "__main__":
    world = world_from_file()
    print(SIZE)
    print_world(world)
    #  world = random_new_world()
    #  print_world(world)
    #  world = generate(world)
    #  print_world(world)
