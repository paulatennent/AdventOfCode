import sys

def slope(xAdd, yAdd, lst):
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
    print(count)
    return count


# input in lines
wholeInput = sys.stdin.read()
lst = wholeInput.split("\n")
length = len(lst)
width = len(lst[0])

total = 1

for i in [1, 3, 5, 7]:
    total = total * slope(i, 1, lst)
    
total = total * slope(1, 2, lst)

print(total)