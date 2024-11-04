Installation and setup
----------------------
Installation instructions can be found in the main Certora Documentation:

* `Sunbeam installation instructions`_
* `Sunbeam Troubleshooting`_


The project
-----------
A brief description of this Rust project directory's contents:

* :clink:`/src/lib.rs` has a Soroban smart contract with some functionality of a Token.
* :clink:`/confs/` has several configuration files to help you run Certora Sunbeam.
* :clink:`/src/certora/` is where we will write the formal specs for this contract.
  It also has a directory of :file:`mutants` to evaluate the specs.


Solutions
---------
This repository has a branch ``solutions``. On this branch, in file
:file:`solutions/solution_specs.rs` you'll find the solutions to all exercises of this
repository. You can consult it if you want to know the answers.


.. Links
   =====

.. _Sunbeam installation instructions:
   https://docs.certora.com/en/latest/docs/sunbeam/installation.html

.. _Sunbeam Troubleshooting:
   https://docs.certora.com/en/latest/docs/sunbeam/troubleshooting.html
