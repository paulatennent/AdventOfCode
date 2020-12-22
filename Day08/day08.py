import sys
wholeInput = sys.stdin.read()

def a():
    acc = 0
    curr = 0
    while (curr < len(lines)):

        if curr in seen:
            return acc
        seen[curr] = acc

        line = lines[curr].split(" ")

        if (line[0] == "nop"):
            curr += 1
        elif (line[0] == "acc"):
            inc = line[1][1:]
            curr += 1
            if (line[1][0] == '+'):
                acc += int(inc)
            else:
                acc -= int(inc)
        elif (line[0] == "jmp"):
            jmp = line[1][1:]
            if (line[1][0] == '+'):
                curr += int(jmp)
            else:
                curr -= int(jmp)

def test(curr2, acc2, ins):

    # change the instruction from nop to jmp OR jmp to nop
    line = lines[curr2].split(" ")
    if (ins == "nop"):
        jmp = line[1][1:]
        if (line[1][0] == '+'):
            curr2 += int(jmp)
        else:
            curr2 -= int(jmp)

    if (ins == "jmp"):
        curr2 += 1

    # continue the program
    while (curr2 < len(lines)):

        # if we find something that has been seen, return false
        if curr2 in seen:
            return False
        seen[curr2] = acc2

        line = lines[curr2].split(" ")
        if (line[0] == "nop"):
            curr2 += 1
        elif (line[0] == "acc"):
            inc = line[1][1:]
            curr2 += 1
            if (line[1][0] == '+'):
                acc2 += int(inc)
            else:
                acc2 -= int(inc)
        elif (line[0] == "jmp"):
            jmp = line[1][1:]
            if (line[1][0] == '+'):
                curr2 += int(jmp)
            else:
                curr2 -= int(jmp)

    # if we got to the end of the prgram, we can return the accumliator
    return acc2
    

def b():
    # make sure a() has be called at least once
    acc = 0
    curr = 0
    while (curr < len(lines)):

        # this hopefully shouldnt happen
        if curr in seen:
            return False

        line = lines[curr].split(" ")

        if (line[0] == "nop"):

            ans = test(curr, acc, "nop")
            if (ans != False):
                return ans

            curr += 1
        elif (line[0] == "acc"):
            inc = line[1][1:]
            curr += 1
            if (line[1][0] == '+'):
                acc += int(inc)
            else:
                acc -= int(inc)
        elif (line[0] == "jmp"):

            ans = test(curr, acc, "jmp")
            if (ans != False):
                return ans

            jmp = line[1][1:]
            if (line[1][0] == '+'):
                curr += int(jmp)
            else:
                curr -= int(jmp)

lines = wholeInput.split("\n")
seen = {}
print(a())
print(b())