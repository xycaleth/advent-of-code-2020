from collections import Counter
from util import read_lines

result = 0
answers = ""
for line in read_lines("../input6.txt"):
    if not line:
        result += len(Counter(answers))
        answers = ""
    else:
        answers += line

print(result)
