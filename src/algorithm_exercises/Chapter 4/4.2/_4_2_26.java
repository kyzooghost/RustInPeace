// 2-satisfiability. 

// Given boolean formula in conjunctive normal form with M clauses and N literals such that each clause has exactly two literals, find a satisfying assignment (if one exists). 

// Hint: Form the implication digraph with 2N vertices (one per literal and its negation). For each clause x + y, include edges from y' to x and from x' to y.

// To satisfy the clause x + y, (i) if y is false,then x is true and (ii) if x is false,then y is true. 
// Claim: The formula is satisfiable if and only if no variable x is in the same strong component as its negation x'. 
// Moreover, a topological sort of the kernel DAG (contract each strong component to a single vertex) yields a satisfying assignment.

/* 

If SCC contains x and !x, not satisfiable
Otherwise reverse topological sort for solution to 2SAT
In this context x + y is x XOR y, which can be represented as two disjunctive statements

x XOR y => (x OR y) AND (!x OR !y)
- x -> y 
- y -> x
- !x -> !y
- !y -> !x

Can model 2-colour problem as 2-SAT with XOR

*/

/*

https://www.youtube.com/watch?v=0nNYy3rltgA

p OR q
p AND q
p + q => Either p or q is true, but not both
!p => Negation

Make compound proposition true
Disjunctions - OR statements
Conjunctions - AND statements
Conjunctive normal form (CNF) => (A or B) and (C or D)...
2-SAT => Only 2 elements in each disjunction (OR statement)
Want to get each disjunction to be true, T & T & T => T
Multiple solutions to 2-SAT

Implication graph
- Edge = Valid inferences
- Node = x or !x
- Make from implication statements

Choose a starting point (x is true, or not true)
- Anything you can reach from starting point, must be valid (because edge is a valid inference rule)
- Chain of implications
- But if follow chain of implications and reach contradiction (x and x! both in chain), not valid chain, then we know this cannot be part of the solution to 2-SAT
- If A and B same implication chain, then (A or B)

Reverse topological order of DAG formed this way
- Can take end of DAG with no consequence

- Or if have DAG (kernal DAG) of strongly connected components (cycles)
- Each strongly connected component = A solution to 2SAT?

- Contradiction where x can reach !x => Cannot satisfy 2SAT
- Check for contradictions

Algorithms
- Find SCCs in graph
- See if x can reach !x for all x => Not satisfiable
- Reverse topological order = solution
    If anything in later SCC conflicts with already taken values, avoid that SCC

*/

public class _4_2_26 {
    
}
