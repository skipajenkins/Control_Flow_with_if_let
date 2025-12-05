
# 🦀 If Let & Pattern Matching in Rust

---

A mini-project exploring enums, match control flow, and the powerful if let syntax in Rust.

This project demonstrates how to:

Build enums with associated data

Implement methods on enums

Use match for pattern matching

Use if let as a cleaner alternative to certain match patterns

Combine pattern matching with logic (e.g., existed_in(year: u16))

---

## 🎯 Learning Objectives

By completing this project, you will understand:

✔️ Enums With Data

How to attach information to enum variants (e.g. Quarter(Ustate)).

✔️ Implementing Methods on Enums

Using impl blocks to give enums behavior like existed_in(year).

✔️ Pattern Matching

Using match to destructure enums and control program flow.

✔️ if let Syntax

A cleaner pattern-matching shorthand for specific enum variants.

✔️ Ownership & Borrowing in Pattern Matching

Using ref to avoid moving values when matching variants.

---

## ⚙️ Setting Up the Environment

Before running the project, ensure Rust is installed.

### 📁 Step 1 — Check installation
```bash
rustc --version
cargo --version
```

If not installed, install Rust using:
```bash
curl https://sh.rustup.rs -sSf | sh
```

Then verify again using the version commands.

### 📁 Step 2 — Clone the Repository
```bash
git clone https://github.com/skipajenkins/revision_3
cd revision_3
```

(Use the actual repo name you plan to give this.)

### ▶️ Step 3 — Run the Project
Build
```bash
cargo build
```
Run
```bash
cargo run
````

---

## 📜 Rust Code (for reference)
```bash
#[derive(Debug)]
enum Ustate {
    Alabama,
    Alaska,
}

impl Ustate {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            Ustate::Alabama => year >= 1819,
            Ustate::Alaska  => year >= 1959,
        }
    }
}

enum Coin {
    Nickel,
    Quarter(Ustate),
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let config_max = Some(3u8);

    // Using `if let` as shorthand for matching a single variant
    if let Some(max) = config_max {
        println!("The maximum is configured to be  {max}");
    }

    let mut count = 6;
    let coin = Coin::Nickel;

    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    let mut count_0 = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count_0 += 1;
    }
}
```

---

## 🧠 Key Concepts Explained
### 🟦 Enum With Data
```bash
enum Coin {
    Nickel,
    Quarter(Ustate),
}
```

This allows each Quarter to carry a state with it.

### 🟥 Implementing Methods on Enums

Rust lets you implement methods directly on enums:
```bash
impl Ustate {
    fn existed_in(&self, year: u16) -> bool { ... }
}
```

This makes enums much more powerful and expressive.

### 🟩 The match Keyword

Used for exhaustive pattern matching:
```bash
match coin {
    Coin::Quarter(ref state) => println!("{state:?}"),
    _ => count += 1,
}
```

### 🟨 if let Syntax

A streamlined way to match a single pattern:
```bash
if let Coin::Quarter(state) = coin {
    println!("{state:?}");
} else {
    count_0 += 1;
}
```

It’s cleaner when you don’t need full exhaustive matching.

### 🟪 Using ref in Pattern Matching

Avoids moving values out of the enum:
```bash
Coin::Quarter(ref state)
```

Without ref, state would be moved, making it unusable afterward.

---

## 📤 Example Output
The maximum is configured to be 3


(Additional output depends on which variants are matched.)

---

## 🦀 Built With

Rust

Cargo

Pattern Matching

Enum Design

---

## 📄 License

This project is open-source under the MIT License.
Feel free to fork, modify, and learn from it.
