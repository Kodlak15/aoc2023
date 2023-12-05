# Advent of Code - Day 4, pt2
```
# Counting all copies of cards attained given an n-card starting pile
# Use HashMap { id: (count, value) } to keep track of results

# Pseudocode:
def pt2():
    # n = number of cards in pile
    hmap = {i: 1 for i in range(n)}

    for id in hmap.keys():
        m = matches(id)
        for j in range(id + 1, id + 1 + m):
            if j in hmap.keys():
                # Add 1 copy of hmap[j] for each copy of hmap[id]
                hmap[j] += hmap[id]  
    
    return hmap.values().sum()

# Test case:
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 -> 4 matches
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19 -> 2 matches
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1 -> 2 matches
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83 -> 1 matches
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36 -> 0 matches
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11 -> 0 matches
result = 30

1) Initiate HashMap { 
    id: count 
}

2) Update counts

$ hmap = { 
    ----------
    1: 1, 
    2: 1,  
    3: 1, 
    4: 1, 
    5: 1, 
    6: 1, 
}

# Card 1:
$ hmap = { 
    1: 1, 
    ----------
    2: 2,  
    3: 2, 
    4: 2, 
    5: 2, 
    6: 1, 
}

# Card 2:
$ hmap = { 
    1: 1, 
    2: 2,  
    ----------
    3: 4, 
    4: 4, 
    5: 2, 
    6: 1, 
}

# Card 3:
$ hmap = { 
    1: 1, 
    2: 2,  
    3: 4, 
    ----------
    4: 8, 
    5: 6, 
    6: 1, 
}

# Card 4:
$ hmap = { 
    1: 1, 
    2: 2,  
    3: 4, 
    4: 8, 
    ----------
    5: 14, 
    6: 1, 
}

# Card 5:
$ hmap = { 
    1: 1, 
    2: 2,  
    3: 4, 
    4: 8, 
    5: 14, 
    ----------
    6: 1, 
}

# Card 6:
$ hmap = { 
    1: 1, 
    2: 2,  
    3: 4, 
    4: 8, 
    5: 14, 
    6: 1, 
    ----------
}

3) Compute sum of counts and return result

Sum = 1 + 2 + 4 + 8 + 14 + 1 = 30
```
