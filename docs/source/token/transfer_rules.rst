2. Effect of transfer on the balances of various addresses
==========================================================

What should be the effect of a ``transfer`` of ``amount`` between two addresses,
``to`` and ``from``?
Write a rule to capture the correct behavior. Whose balance should change by what amount?

Note that ``transfer`` should not affect any address other than the one being transferred
to and from. Write another rule to encode the effect of ``transfer`` on some ``other``
address. 

You can write these two properties in ``transfer_is_correct`` and
``transfer_no_effect_on_other`` in
:clink:`src/certora/spec.rs <@token-proj/src/certora/spec.rs>`.

Once you have written the rule, you can run Certora Sunbeam to check it by running:

.. code-block:: bash

   certoraSorobanProver exercise2.conf

from the :rust:`projects/token/confs` directory.

.. dropdown:: Solution

   .. cvlinclude:: @token-proj/solutions/solution_specs.rs
      :language: rust
      :start-at: Exercise 2
      :end-before: Exercise 3
      :caption:
