5. Assessing your specs through mutation testing
================================================

How do you know if your rules are good enough to catch potential bugs?
One technique is called "mutation testing" where small faults are injected in to the
program and it is checked against the same rule. Verification should fail if the rule
is good at catching the fault. If verification passes, that means your rule has gaps
that must be addressed.

We have provided 3 hand-written mutants in
:clink:`src/certora/mutants  <@token-proj/src/certora/mutants>`.
Copy them one at a time to :file:`src/` and rename them to `lib.rs` to replace the
original :clink:`src/lib.rs <@token-proj/src/lib.rs>`.
Then run the rules you wrote above for ``transfer`` on these mutants. Are they caught?

Can you detect what the mutation was, for each mutant?
You can see the solution in :file:`solutions/bugs-in-mutants.md`.


Note that there are other ways to assess the quality of your rule.
You can mutate the rule to see if it is vacuous, you can check if the rule is a
tautology, and you can use UNSAT cores to understand what parts of the code were
covered by the rule.

.. dropdown:: Mutant 1. Solution

   The bug is that the value read when the :rust:`addr` key is not present in storage is
   :rust:`1` but it should be :rust:`0`.

   .. code-block:: rust

      pub fn read_balance(e: &Env, addr: &Address) -> i64 {
          e.storage().persistent().get(&addr).unwrap_or(1)
      }

.. dropdown:: Mutant 2. Solution

   The bug is that :rust:`transfer` calls :rust:`spend_balance` twice instead of
   calling it once followed by a :rust:`receive_balance`.

   .. code-block:: rust

      pub fn transfer(e: &Env, from: Address, to: Address, amount: i64) {
          from.require_auth();
          check_nonnegative_amount(amount);
          spend_balance(&e, from.clone(), amount);
          spend_balance(&e, to.clone(), amount);
      }

.. dropdown:: Mutant 3. Solution

   The bug is that :rust:`transfer` calls
   :rust:`spend_balance(&e, from.clone(), amount + 1);`
   instead of :rust:`spend_balance(&e, from.clone(), amount);`.

   .. code-block:: rust

      pub fn transfer(e: &Env, from: Address, to: Address, amount: i64) {
          from.require_auth();
          check_nonnegative_amount(amount);
          spend_balance(&e, from.clone(), amount + 1);
          receive_balance(&e, to.clone(), amount);
      }
