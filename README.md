# Combination Lock Digit Normalizer

A Rust CLI utility designed to obscure combination lock passwords by calculating the nearest single repeating digit (e.g., `88888`). Setting all dials on a combination lock to the same digit makes it significantly harder for onlookers or casual observers to guess your code, while minimizing the physical effort (total dial spins) required to scramble and reset it.

> ℹ️ **Authorship & AI Usage Disclosure**
> 
> **The vast majority of this codebase—including the main application flow, input validation, output table formatting, and custom algorithms—was designed and implemented by me. AI (Google Gemini) was used as a tool to benchmark performance, design output formatting, and write documentation.**
>* **Human Contributions:**
>   * Project concept, architecture, and Rust CLI implementation.
>   * Terminal user input parsing, string sanitization, and input validation.
>   * Primary algorithm implementations: **Naive Average** and **Distance Average** located in `src/main.rs`.
>* **AI Contributions (Google Gemini):**
>   * **Optimal Min-Distance Algorithm (`src/gemini.rs`)**: Generated to serve as a mathematically optimal $O(N)$ benchmark to test and validate the human-written heuristics.
>   * **Spin Table Formatting**: Formatted print logic displaying dial spin offsets.
>   * **README File**: Drafted by Gemini based on the codebase and prompt instructions.

---

## ⚙️ How It Works

The program compares three calculation strategies:

1. **Naive Average** *(Human)*: Computes the standard arithmetic mean of the combination's digits and repeats the resulting integer.
2. **Distance Average** *(Human)*: Calculates relative circular distance from each dial to all other dials to produce an intermediate array before returning a target digit.
3. **Optimal Min-Distance** *(AI Benchmark)*: Checks all candidate target digits ($0$ through $9$) using modular shortest dial distance to guarantee the globally optimal minimum rotation cost.

---

## 🚀 Getting Started

### Prerequisites

* [Rust and Cargo](https://www.rust-lang.org/) installed on your machine.
* `make` (GNU Make) utility.

### Building & Running

This project includes a `Makefile` to streamline building and testing the release executable.

* **Build the release binary:**
```bash
  make build
  # or simply: make
```

* **Run the application:**
```bash
make test
```


* **Clean build artifacts:**
```bash
make clean
```

*(Alternatively, you can build and run directly via Cargo with `cargo run --release`)*

---

## 📊 Sample Output

### Example 1: Standard Combination

```text
Input your combination:
==> 12345

Using naive average: "33333"
Using distance average: "33333"
Using optimal min-distance: "33333"
=====================
Original | Spin | New
1        | 2    | 3
2        | 1    | 3
3        | 0    | 3
4        | -1   | 3
5        | -2   | 3
=====================
Total spins: 6

```

### Example 2: Edge Case Comparison

In cases where digits cluster around the $0$/$9$ dial boundary, heuristic averages can diverge from the true minimal rotation target:

```text
Input your combination:
==> 56198165189

Using naive average: "55555555555"
Using distance average: "55555555555"
Using optimal min-distance: "88888888888"
=====================
Original | Spin | New
5        | 3    | 8
6        | 2    | 8
1        | -3   | 8
9        | -1   | 8
8        | 0    | 8
1        | -3   | 8
6        | 2    | 8
5        | 3    | 8
1        | -3   | 8
8        | 0    | 8
9        | -1   | 8
=====================
Total spins: 21

```

---

## 📁 Project Structure

* **`src/main.rs`**: Core program execution, input validation, output formatting, and custom human-written heuristic algorithms (`naive_avg`, `distance`).


* **`src/gemini.rs`**: Circular dial math helpers (`shortest_spin`, `total_dial_distance`) and AI-generated benchmark function (`optimal_distance`).
