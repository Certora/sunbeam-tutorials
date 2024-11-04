5. Assessing your specs through mutation testing
================================================

How do you know if your rules are good enough to catch potential bugs?
One technique is called "mutation testing" where small faults are injected in to the
program and it is checked against the same rule. Verification should fail if the rule
is good at catching the fault. If verification passes, that means your rule has gaps
that must be addressed.

We have provided 3 hand-written mutants in :clink:`/src/certora/mutants`.
Copy them one at a time to `src` and rename them to `lib.rs` to replace the
original :clink:`/src/lib.rs`.
Then run the rules you wrote above for ``transfer`` on these mutants. Are they caught?

Can you detect what the mutation was, for each mutant?
You can see the solution in :file:`solutions/bugs-in-mutants.md`.


Note that there are other ways to assess the quality of your rule.
You can mutate the rule to see if it is vacuous, you can check if the rule is a
tautology, and you can use UNSAT cores to understand what parts of the code were
covered by the rule.
