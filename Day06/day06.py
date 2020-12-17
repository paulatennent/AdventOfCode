import sys
import re 

# read in inputs
wholeInput = sys.stdin.read()

# sepeate by double \n
groups = wholeInput.split("\n\n")


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

# add to total
print(total)