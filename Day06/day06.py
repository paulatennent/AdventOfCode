import sys
import re

# first subquestion
def a():
    # add into a set
    total = 0
    for group in groups:
        questions = set()
        for char in group:
            #print(char)
            if (char != "\n"):
                questions.add(char)

        # count set
        total += len(questions)

    return total

# second subquestion
def b():

# add into a set
    total = 0
    for group in groups:
        questions = {}
        people = group.split("\n")
        for person in people:
            for char in person:
                if (char != "\n"):
                    if char in questions:
                        questions[char] += 1
                    else:
                        questions[char] = 1 

        # count set
        for item in questions:
            if (questions[item] == len(people)):
                total += 1
    return total

# read in inputs
wholeInput = sys.stdin.read()

# sepeate by double \n
groups = wholeInput.split("\n\n")

print(a())
print(b())