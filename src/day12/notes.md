# Goal:
```
For each row, count all of the different arrangements of operational and broken springs that meet the given criteria. 
What is the sum of those counts?
```

# Notes:
- let n = the number of character groupings in a line containing only '?' or '#'
- let g = the number of groups in a line (comma separated numbers on right side of line)
- for every line, g >= n
- each group of broken coils must all be grouped as a consecutive sequence of broken coils
    - correct grouping:   ..###
    - incorrect grouping: .#.##

# g = n
- each of the n character groupings corresponds directly to the size at that same index
```
Example: .??..??...?##. 1,1,3

- imaging there are 3 sliding windows of lengths 1, 1, and 3 which each correspond to the three distinct groups
- the sliding windows are composed of consecutive known broken coils '#' and possibly broken coils '?'
- determine the number of times each sliding window can be shifted to the right by 1 index
- this number + 1 represents the number of possible arrangements for each distinct group
- the total number of possible arrangements is the product the possible arrangements for each distinct group
- in the event a distinct group contains known broken coils equal to the length of the window, there is only one possible arrangement

=> group 1 can be shifted right once, so there are 1 + 1 = 2 possible arrangements
=> group 2 can be shifted right once, so there are 1 + 1 = 2 possible arrangements
=> group 3 cannot be shifted to the right, so there are 0 + 1 = 1 possible arrangements
=> total = 2 * 2 * 1 = 4 possible arrangements
```

# g > n
- If the number of groups exceeds the number of character groupings, that means there are sub-groups that need to be found
- If possible regions overlap (groups 2 and 3 here), then the position of the rightmost broken coil in the group on the left can shrink the region of possibilities of the group on the right
```
Example: ?###???????? 3,2,1

- use sliding windows to find all group combinations, and determine which are possible
- from left to right, start by determining the range of possibilities for each group
- if a number of consecutive broken coils is found equal to the leftmost (moving) window, stop shifting that window there
- once the range of possibilities is established, find all possible combinations

     0 1 2 3 4 5 6 7 8 9 t e
t1:  ? # # # ? ? ? ? ? ? ? ? (not possible)
g1: [     ]
g2:         [   ]
g3:               [ ]

- shift groups 1, 2, and 3 to the right by 1
- here we find 3 consecutive known broken coils, so we know t1 is impossible

     0 1 2 3 4 5 6 7 8 9 t e
t2:  ? # # # ? ? ? ? ? ? ? ?
g1:   [     ]
g2:           [   ]
g3:                 [ ]

- shift groups 2, and 3 to the right by 1

     0 1 2 3 4 5 6 7 8 9 t e
t3:  ? # # # ? ? ? ? ? ? ? ?
g1:   [     ]
g2:             [   ]
g3:                   [ ]

- shift groups 2, and 3 to the right by 1

     0 1 2 3 4 5 6 7 8 9 t e
t4:  ? # # # ? ? ? ? ? ? ? ?
g1:   [     ]
g2:               [   ]
g3:                     [ ]

- shift groups 2, and 3 to the right by 1
- here we have established the range of possibilities for all groups

     0 1 2 3 4 5 6 7 8 9 t e
t5:  ? # # # ? ? ? ? ? ? ? ?
g1:   [     ]
g2:                 [   ]
g3:                       [ ]

- move group 2 back to the start of its range and iterate over all possibilities for group 3 


     0 1 2 3 4 5 6 7 8 9 t e
t6:  ? # # # ? ? ? ? ? ? ? ?
g1:   [     ]
g2:           [   ]
g3:                 [ ]

     0 1 2 3 4 5 6 7 8 9 t e
t7:  ? # # # ? ? ? ? ? ? ? ?
g1:   [     ]
g2:           [   ]
g3:                   [ ]

     0 1 2 3 4 5 6 7 8 9 t e
t8:  ? # # # ? ? ? ? ? ? ? ?
g1:   [     ]
g2:           [   ]
g3:                     [ ]

     0 1 2 3 4 5 6 7 8 9 t e
t9:  ? # # # ? ? ? ? ? ? ? ?
g1:   [     ]
g2:           [   ]
g3:                       [ ]

- shift group 2 to the right by 1 and repeat the previous process
- keep in mind the starting point for group 3 if shifted to the right each time group 2 if shifted to the right

     0 1 2 3 4 5 6 7 8 9 t e
tt:  ? # # # ? ? ? ? ? ? ? ?
g1:   [     ]
g2:             [   ]
g3:                   [ ]

     0 1 2 3 4 5 6 7 8 9 t e
te:  ? # # # ? ? ? ? ? ? ? ?
g1:   [     ]
g2:             [   ]
g3:                     [ ]

...
...
...

- the tree of possibilities looks something like this:

{
    [1-3]: 
    {
        [5-6]: 
        {
            [8],
            [9],
            [10],
            [11],
        },
        [6-7]: 
        {
            [9],
            [10],
            [11],
        },
        [7-8]: 
        {
            [10],
            [11],
        },
        [8-9]: 
        {
            [11],
        },
    }, 
}

- the total number of arrangements is equal to the number of total number of leaves on the tree
- ie: 4 + 3 + 2 + 1 = 10
```
