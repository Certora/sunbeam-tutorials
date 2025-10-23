.. index:: mutation

5. Assessing your specs through mutation testing
================================================

How do you know if your rules are good enough to catch potential bugs?
One technique for assessing the quality of a specification is called "mutation testing".
Small modifications (mutations) are intentionally made, one
at a time, in the contract source code to cause logic faults. The modified code is checked 
using your existing rules. It is important that your rules pass with the original 
contract code before applying any mutations so that you can safely draw conclusions from
the mutation tests. With a mutation in the code, verification 
will fail if the coverage of your rule set is good enough to catch the fault. If 
verification still passes, that likely means your rules have a gap that must be addressed.

We have provided 3 hand-written mutants in
:clink:`src/certora/mutants  <@token-proj/src/certora/mutants>`.
Copy them one at a time to :file:`src/` and rename them to `lib.rs` to replace the
original :clink:`src/lib.rs <@token-proj/src/lib.rs>`.
Then run the rules you wrote above for ``transfer`` on these mutants. Are they caught?

Can you detect what the mutation was, for each mutant?
You can see the solution in :file:`solutions/bugs-in-mutants.md`.


.. todo:: The following text needs more explanation / links:
  Note that there are other ways to assess the quality of your rule.
  You can mutate the rule to see if it is vacuous, you can check if the rule is a
  tautology, and you can use ``UNSAT`` cores to understand what parts of the code were
  covered by the rule.

.. dropdown:: Mutant 1. Solution

   The bug is that the value read when the :rust:`addr` key is not present in storage is
   :rust:`1` but it should be :rust:`0`.

   .. cvlinclude:: @token-proj/src/certora/mutants/mutant1.rs
      :language: rust
      :start-at: pub fn read_balance
      :end-before: fn write_balance

.. dropdown:: Mutant 2. Solution

   The bug is that :rust:`transfer` calls :rust:`spend_balance` twice instead of
   calling it once followed by a :rust:`receive_balance`.

   .. cvlinclude:: @token-proj/src/certora/mutants/mutant2.rs
      :language: rust
      :start-at: pub fn transfer
      :end-before: pub fn mint
      :emphasize-lines: 4, 5

.. dropdown:: Mutant 3. Solution

   The bug is that :rust:`transfer` calls
   :rust:`spend_balance(&e, from.clone(), amount + 1);`
   instead of :rust:`spend_balance(&e, from.clone(), amount);`.

   .. cvlinclude:: @token-proj/src/certora/mutants/mutant3.rs
      :language: rust
      :start-at: pub fn transfer
      :end-before: pub fn mint
      :emphasize-lines: 4
