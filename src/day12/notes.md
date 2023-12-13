# Goal:
```
For each row, count all of the different arrangements of operational and broken springs that meet the given criteria. 
What is the sum of those counts?
```

# Notes:
- let n = the number of character groupings in a line containing only '?' or '#'
- let g = the number of groups in a line (comma separated numbers on right side of line)
- for every line, g >= n

### g = n
- each of the n character groupings corresponds directly to the size at that same index
```
Example: .??..??...?##. 1,1,3
=> The first value (1) corresponds to the first ??
=> The second value (1) corresponds to the second ??
=> The third value (3) corresponds to ?##

In the first group, there is 1 broken spring that can be in one of two positions. Thus, there are 2 possible
arrangements in that group. The same logic applies to the second group. In the third group, there are 3 broken
springs, two of which are already known. Thus, there is only 1 possible arrangement in that group. To get the total
number of possible arrangements, multiply the possible arrangements for each group together to get 2 * 2 * 1 = 4.

Example: #???? 3
- b = 3 broken springs
- k = 1 known location
- u = (b - k) = 2 unknown locations
- t = 5 possible positions
- a = (t - k) = 4 available positions

The goal is to find the number of ways we can arrange 2 broken springs in 4 possible positions.

arrangements = a! / u! = 4! / 2! = 4 * 3 = 12 
```

### g > n
- If the number of groups exceeds the number of character groupings, that means there are sub-groups that need to be found
```

```
