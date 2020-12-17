import sys

def slope(xAdd, yAdd):
    # count number of trees
    count = 0

    # start at top left
    x = 0
    y = 0
    # while not at the bottom
    while (y < length):
        # check if its a tree
        #print(x, y)
        if lst[y][x] == '#':
            count += 1

        # if its at the right, move back to the left
        x += xAdd
        x = x % width
        y += yAdd
    return count

# first subset, find how many trees collide with a slope of down 1, across 3
def a():
    return slope(3, 1)

def b():
    total = 1
    for i in [1, 3, 5, 7]:
        total = total * slope(i, 1)
        
    total = total * slope(1, 2)
    return total


# input in lines
wholeInput = sys.stdin.read()
lst = wholeInput.split("\n")
length = len(lst)
width = len(lst[0])

print(a())
print(b())