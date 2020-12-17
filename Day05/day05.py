import sys
import re 

def a():
    maxId = 0
    idSet = set()

    # for each pass
    for i in range(0, len(passes)):
        #seperate into row and coll
        rowBin = passes[i][0:7]
        rowBin = re.sub("F", "0", rowBin)
        rowBin = re.sub("B", "1", rowBin)
        rowBin = re.sub("0*", "", rowBin, 1)
        if (rowBin == ""):
            rowBin = "0"

        colBin = passes[i][7:10]
        colBin = re.sub("L", "0", colBin)
        colBin = re.sub("R", "1", colBin)
        colBin = re.sub("0*", "", colBin, 1)
        if (colBin == ""):
            colBin = "0"

        row = int(rowBin, 2)
        col = int(colBin, 2)

        id = row * 8 + col
        idSet.add(id)

        maxId = max(id, maxId)
    return maxId

def b():
    maxId = 0
    idSet = set()

    # for each pass
    for i in range(0, len(passes)):
        #seperate into row and coll
        rowBin = passes[i][0:7]
        rowBin = re.sub("F", "0", rowBin)
        rowBin = re.sub("B", "1", rowBin)
        rowBin = re.sub("0*", "", rowBin, 1)
        if (rowBin == ""):
            rowBin = "0"

        colBin = passes[i][7:10]
        colBin = re.sub("L", "0", colBin)
        colBin = re.sub("R", "1", colBin)
        colBin = re.sub("0*", "", colBin, 1)
        if (colBin == ""):
            colBin = "0"

        row = int(rowBin, 2)
        col = int(colBin, 2)

        id = row * 8 + col
        idSet.add(id)

        maxId = max(id, maxId)

    first = False
    for i in idSet:
        if (not first):
            first = True
        elif (i != prev + 1):
            return i - 1
        prev = i


# input in lines
wholeInput = sys.stdin.read()
passes = wholeInput.split("\n")

print(a())
print(b())