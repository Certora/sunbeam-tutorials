1. A property to check the initial balance of a new account
===========================================================

What should be the balance of a new address?
Write a property to check that this is indeed the balance of a new address.
You can write your property (or rule) in
:clink:`src/certora/spec.rs <@token-proj/src/certora/spec.rs>` inside the function
:rust:`init_balance`. We have already provided the right signature for this rule.

Once you have written the rule, you can run Certora Sunbeam to check it by running:


.. code-block:: bash

   certoraSorobanProver exercise1.conf

from the :rust:`projects/token/confs` directory.

.. dropdown:: Hint

   You'll need to use :rust:`require!(CONDITION, "address must not exists");`
   to ensure the *address* does not already exist.  

.. dropdown:: Solution

   .. cvlinclude:: @token-proj/solutions/solution_specs.rs
      :language: rust
      :start-at: Exercise 1
      :end-before: Exercise 2
      :caption:
