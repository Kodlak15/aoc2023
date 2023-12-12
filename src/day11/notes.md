# Cannot hold array in memory for part 2
```
Strategy:
- let the growth factor of empty rows and columns = k
- let p1, p2 represent two arbitrarily chosen points in the universe
- let the number of non-empty rows and columns between p1 and p2 = nr, nc respectively
- let the number of empty rows and columns between p1 and p2 = er, ec respectively
- distance(p1, p2) = nr + nc + (k * (er + ec))
- need to find a way to calculate nr, nc, er, ec to compute result
```
