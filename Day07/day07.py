import sys
import re 


def dfs(curr, seen, bagGra):
    # if the item has been seen in the dfs, skip
    if curr in seen:
        return 0
    
    # set the current bag to be seen
    seen.append(curr)
    total = 1

    # for each bag that holds the current bag, do a recursive call
    if curr in bagGra:
        for bag in bagGra[curr]:
            total += dfs(bag, seen, bagGra)
    return total

# create graph
bagGra = {}

# insert in data
wholeInput = sys.stdin.read()

# split up by line
lines = wholeInput.split("\n")

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
        if bag in bagGra:
            bagGra[bag].append(fstBag)
        else:
            bagGra[bag] = [fstBag]

    #print(bagGra)

# go from shiny gold bag and do a bfs to find all bags inside
seen = []
total = dfs("shiny gold", seen, bagGra)
print(total - 1)