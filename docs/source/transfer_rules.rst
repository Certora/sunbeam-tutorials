2. Effect of transfer on the balances of various addresses
==========================================================

What should be the effect of a ``transfer`` of ``amount`` between two addresses,
``to`` and ``from``?
Write a rule to capture the correct behavior. Whose balance should change by what amount?

Note that ``transfer`` should not affect any address other than the one being transferred
to and from. Write another rule to encode the effect of ``transfer`` on some ``other``
address. 

You can write these two properties in ``transfer_is_correct`` and
``transfer_no_effect_on_other`` in :clink:`/src/certora/spec.rs`.

Once you have written the rule, you can run Certora Sunbeam to check it by running:

.. code-block:: bash

   certoraRun confs/exercise2.conf

----

3. ``transfer`` under insufficient funds
========================================

If ``from`` does not have sufficient balance, ``transfer`` of funds should not succeed.
Write a rule to capture this behavior in the function ``transfer_fails_if_low_balance``.

Once you have written the rule, you can run Certora Sunbeam to check it by running:


.. code-block:: bash

   certoraRun confs/exercise3.conf
