3. ``transfer`` under insufficient funds
========================================

If ``from`` does not have sufficient balance, ``transfer`` of funds should not succeed.
Write a rule to capture this behavior in the function ``transfer_fails_if_low_balance``.

Once you have written the rule, you can run Certora Sunbeam to check it by running:


.. code-block:: bash

   certoraSorobanProver exercise3.conf

from the :rust:`projects/token/confs` directory.

.. dropdown:: Solution

   .. cvlinclude:: @token-proj/solutions/solution_specs.rs
      :language: rust
      :start-at: Exercise 3
      :end-before: Exercise 4
      :caption:
