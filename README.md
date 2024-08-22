# Sirius Simple Example

Welcome to the sirius-simple-example repository, an example to get you started with the [**Sirius**](https://github.com/snarkify/sirius/) framework for Incrementally Verifiable Computation (IVC).

Built from [sirius-quickstart](https://github.com/snarkify/sirius-quickstart), this repository demonstrates a straightforward implementation of a `StepCircuit` that performs a simple summation operation over multiple folding steps. This example is designed to help developers understand the core concepts and API of Sirius, particularly in configuring and synthesizing custom circuits.

## Introduction

This example showcases the implementation of a custom `StepCircuit` that sums its input (`z_in`) and outputs the result. Unlike the trivial identity mapping example in the original quickstart, this example adds more complexity to illustrate how you can customize and extend the framework to suit your needs.

## Understanding the StepCircuit in Sirius Simple Example

In the sirius-simple-example, the `StepCircuit` performs a simple yet illustrative operation that sums its input values (`z_in`). This specific implementation serves as an educational example of how to define custom step-circuits within the Sirius framework.

### What Does the StepCircuit Do?
The `StepCircuit` in this example is designed to perform a summation operation. Here's what it does in detail:

- Input Handling (`z_in`): The circuit takes an input, denoted as `z_in`. This input is an array of field elements that represent the data from the previous computation step or the initial input for the circuit (`z_0`).

- Summation Logic: The circuit then creates a gate that sums each element in `z_in` with itself (i.e., performs input + input) and outputs the result. Essentially, this operation doubles each element of the input array.

- Output (`z_out`): The result of this summation is placed in an output array, `z_out`, which is then passed to the next step of the circuit. In this example, `z_out` is simply `2 * z_in` for each element, which demonstrates the basic use of gates and selectors in a custom circuit.

In a real-world scenario, such a circuit could be extended to perform more complex arithmetic or logic operations, depending on the application's needs. However, the simplicity of this example makes it an excellent starting point for developers new to the framework.

## Prerequisites

### 1. Install Rust

If you haven't already installed Rust, you can do so by using [rustup](https://rustup.rs/). Rustup will set up your environment with the latest stable Rust compiler and Cargo, Rust's package manager.

To install Rust, run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, make sure your Rust toolchain is up-to-date:

```bash
rustup update
```

### 2. Clone the Repository
Clone the sirius-quickstart repository to your local machine:

```bash
git clone https://github.com/your-username/sirius-simple-example.git
cd sirius-simple-example
```

## Project Structure
The project structure is as follows:

- src/: Contains the source code for the example.
- Cargo.toml: The Cargo configuration file, listing dependencies and metadata for the project.

## Running the Example

### 1. First Run

To run the example for the first time, use the following command:

```bash
cargo run --release
```

This will compile the project in release mode, which is optimized for speed. During this initial run, the commitment keys for the BN256 and Grumpkin curves will be generated and cached. This process may take some time, so running in release mode ensures it completes as quickly as possible.

### 2. Subsequent Runs
For subsequent runs, you can use the following command without the --release flag:

```bash
cargo run
```

This will reuse the previously generated commitment keys, so the process will be faster, and there’s no need to recompile in release mode unless you're making significant changes or need the performance optimization again.

### 3. Expected Output
When the example runs successfully, you should see output indicating that the folding steps were executed and verified successfully:

```text
start setup primary commitment key: bn256
start setup secondary commitment key: grumpkin
ivc created
folding step 1 was successful
folding step 2 was successful
folding step 3 was successful
folding step 4 was successful
folding step 5 was successful
verification successful
success
```

## Understanding the Example
This example demonstrates the following key concepts of the Sirius framework:

- StepCircuit: A trait representing the circuit for each step in the IVC. In this example, the circuit performs an identity operation.
- Commitment Keys: Setup for the primary and secondary circuits, using BN256 and Grumpkin elliptic curves.
- Folding Steps: Execution of multiple folding steps, each represented by an invocation of the fold_step function.

For more detailed explanations, please refer to the main [Sirius documentation](https://docs.snarkify.io/sirius-folding/quickstart).

## Next Steps
After understanding this basic example, you can explore more complex examples and customize your circuits:

- Modify the StepCircuit implementation to perform non-trivial operations.
- Experiment with different folding step counts and configurations.
- Explore the Sirius main repository for advanced features like custom gates and high-degree optimizations.

# Getting Involved

We'd love for you to be a part of our community!

If you're as enthusiastic about `Sirius` as we are, we invite you to join our developer community at Telegram. It's a great place to stay updated, get involved, and contribute to the project. Whether you're looking to contribute code, provide feedback, or simply stay in the loop, our Telegram group is the place to be.

:point_right: [Join our developer community](https://t.me/+oQ04SUgs6KMyMzlh)

Thank you for your interest in our project! :sparkles:
