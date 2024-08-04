# Body problem

A simulation of the [n-body problem](https://en.wikipedia.org/wiki/N-body_problem) (which is a generalization of, for
instance, the three-body problem) in a web application. It allows the user to set up the bodies and their properties.

This Cargo workspace consists of

- a library crate [body_problem](/body_problem) that provides the simulation logic and
- a binary crate [body_problem_web_app](/body_problem_web_app) utilizing the library and displaying the simulation.

The app is deployed at [body-problem.matousvolf.cz](https://body-problem.matousvolf.cz).
