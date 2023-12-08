# Rules
- Check hand type first
```
five of a kind > four of a kind > full house > three of a kind > two pair > one pair > high card
```
- If the hand type is the same, the hand with the more valuable card at the leftmost index where the cards do not equal one another wins
```
A > K > Q > J > T > 9 > 8 > 7 > 6 > 5 > 4 > 3 > 2

Example:
77878 > 77788
```
- Rank the cards
```
# If there are 5 cards:
Greatest value card gets rank 5 ... Lowest value card gets rank 1
```
- Winnings are the sum of the products of each hands rank with the associated bid for that hand

# Notes
- In Rust, you can implement PartialOrd on custom data structure (such as enums and structs) to get ordering behavior
- Additionally, if you have one such data structure nested within another, Rust is able to use the ordering of the interior
data structure to break any ties that occur in the parent 
- In this particular case, I was able to sort my hands in ascending order, and the cards within were automatically handled when doing so
