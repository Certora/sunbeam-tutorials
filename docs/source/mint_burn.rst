4. Specs for ``mint`` and ``burn``
==================================

Now that we have seen rules for ``transfer``, think of some properties for
``mint`` and ``burn`` and write them in the :clink:`/src/certora/spec.rs` file.
To run them, create your own :file:`.conf` files under :clink:`/confs` by looking at the
existing conf files. You will only need to change the names of the rules passed into the
``"rule"`` field. The rest should be the same.

You can see several rules we have written for these functions in
:file:`solutions/solution_specs.rs` of branch ``solutions``.
