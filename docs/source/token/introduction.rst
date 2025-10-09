Introduction
============

This tutorial builds upon concepts covered in the main Certora Prover Tutorials such 
as propositional & predicate logic, and the basics of formal verification. If these 
are not already familiar to you, refer to `section 1 of the Certora Prover Tutorials`_ 
before proceeding.


What is Certora Sunbeam?
------------------------
Sunbeam is a tool for formally verifying Soroban smart contracts written in Rust. You can 
leverage Certora's Cavelier (CVLR) library to write specifications for such contracts in Rust also.
You can read more about Sunbeam in the `User Guide for Sunbeam`_ documentation.


The project
-----------
The tutorial project's directory is in :clink:`projects/token <@token-proj/>`.
Below is a brief description of this Rust project directory's contents:

* :clink:`src/lib.rs <@token-proj/src/lib.rs>` has a Soroban smart contract with some
  functionality of a Token.
* :clink:`confs/ <@token-proj/confs/>` has several configuration files to help you run Certora Sunbeam.
* :clink:`src/certora/ <@token-proj/src/certora/>` is where we will write the formal specs for this contract.
  It also has a directory of :file:`mutants` to evaluate the specs.


Solutions
---------
In file :clink:`solutions/solution_specs.rs <@token-proj/solutions/solution_specs.rs>`
you'll find the solutions to all exercises of this repository.
You can consult it if you want to know the answers.

.. Links
   =====

.. _section 1 of the Certora Prover Tutorials:
   https://docs.certora.com/projects/tutorials/en/latest/lesson1_prerequisites/index.html
.. _User Guide for Sunbeam:
   https://docs.certora.com/en/latest/docs/sunbeam/usage.html#user-guide-for-sunbeam