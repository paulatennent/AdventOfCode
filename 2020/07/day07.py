import sys
import re 


def dfsa(curr, seen, bagGra):
    # if the item has been seen in the dfs, skip
    if curr in seen:
        return 0
    
    # set the current bag to be seen
    seen.append(curr)
    total = 1

    # for each bag that holds the current bag, do a recursive call
    if curr in bagGra:
        for bag in bagGra[curr]:
            total += dfsa(bag, seen, bagGra)
    return total

def dfsb(curr, bagGra):
    
    # set the current bag to be seen
    total = 1

    if curr[0] in bagGra:
        for bag in bagGra[curr[0]]:
            add = dfsb(bag, bagGra) * int(bag[1])
            total += add

    return total


def a():
    # create graph
    grapha = {}
    # find first bag and list of bags inside
    for line in lines:
        fstBag = re.findall("\w* \w*", line)
        if (len(fstBag) > 1):
            fstBag = fstBag[0]
        else:
            continue
        #print(fstBag)

        oth = re.sub(".*contain ", "", line)
        oth = re.sub(".*no other.*", "", oth)
        oth = re.sub("\d ?", "", oth)
        oth = re.sub("\.", "", oth)
        oth = re.sub("bags*", "", oth)
        oth = re.findall("\w+ \w+", oth)
        #print(oth)

        # connect first bag to all bags in list (going backwards)
        for bag in oth:
            if bag in grapha:
                grapha[bag].append(fstBag)
            else:
                grapha[bag] = [fstBag]

    # go from shiny gold bag and do a bfs to find all bags inside
    seen = []
    total = dfsa("shiny gold", seen, grapha)
    return total - 1

def b():

    graphb = {}
    # find first bag and list of bags inside
    for line in lines:
        fstBag = re.findall("\w* \w*", line)
        if (len(fstBag) > 1):
            fstBag = fstBag[0]
        else:
            continue
        #print(fstBag)

        oth = re.sub(".*contain ", "", line)
        oth = re.sub(".*no other.*", "", oth)
        oth = re.sub("\.", "", oth)
        oth = re.sub("bags*", "", oth)
        oth = re.findall("\d+ \w+ \w+", oth)
        #print(oth)

        # connect first bag to all bags in list (going backwards)
        for bag in oth:
            num = re.findall("\d+", bag)[0]
            bag = re.sub("\d ?", "", bag)
            #print(num, bag)
            item = [bag, num]
            if fstBag in graphb:
                graphb[fstBag].append(item)
            else:
                graphb[fstBag] = [item]

    # go from shiny gold bag and do a bfs to find all bags inside
    seen = []
    total = dfsb(["shiny gold", 1], graphb)
    return total - 1


# insert in data
wholeInput = sys.stdin.read()

# split up by line
lines = wholeInput.split("\n")

print(a())
print(b())
