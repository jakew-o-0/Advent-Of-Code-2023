import pprint

loc_sets = []
nums = []
dest = 0

with open("./input.txt", 'r') as file:
    nums = file.readline().split(' ')
    sets = []   

    for line in file:
        if line.isspace():
            print(sets)
            loc_sets.append(sets)
            sets.clear()
            continue
        line = line.rstrip('\n')
        sets.append(line.split(' '))

    loc_sets.append(sets)
    
pprint.pprint(loc_sets)