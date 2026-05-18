// ============================================================
// CHALLENGE EXERCISES — Ownership, Borrowing & References
// ============================================================

fn main() {
    println!("=== Challenge Exercises ===\n");

    // ----------------------------------------------------------
    // CHALLENGE 1: fn append_signature(report: &mut String)
    // Modify a String in-place via mutable reference
    // ----------------------------------------------------------
    println!("--- Challenge 1: append_signature ---");

    let mut report = String::from("Q1 Sales Report: Revenue up 12%.");
    println!("Before: {}", report);

    append_signature(&mut report); // pass a mutable reference
    println!("After:  {}", report); // original variable is modified!

    // What happened?
    // &mut String lets the function modify the caller's data directly.
    // No ownership transfer — `report` is still valid here.

    // ----------------------------------------------------------
    // CHALLENGE 2: Two mutable references — what error occurs?
    // ----------------------------------------------------------
    println!("\n--- Challenge 2: Two mutable references ---");

    let mut text = String::from("hello");

    let r1 = &mut text;
    // let r2 = &mut text; // ❌ UNCOMMENT to see the error:
    //
    // error[E0499]: cannot borrow `text` as mutable more than once at a time
    //   --> src/main.rs:XX:14
    //    |
    //    | let r1 = &mut text;
    //    |          --------- first mutable borrow occurs here
    //    | let r2 = &mut text;
    //    |          ^^^^^^^^^ second mutable borrow occurs here
    //    | println!("{} {}", r1, r2);
    //    |                   -- first borrow later used here
    //
    // WHY? Rust's borrow checker enforces:
    // "At any given time, you can have EITHER one &mut OR any number of &"
    // This prevents data races at compile time — no runtime cost needed.

    r1.push_str(" world"); // only r1 is active
    println!("Modified via r1: {}", text);

    // Safe way to use two mutations: do them sequentially, not simultaneously
    {
        let r1 = &mut text;
        r1.push_str(" — first edit");
    } // r1 goes out of scope here
    {
        let r2 = &mut text; // ✅ now it's safe, r1 is gone
        r2.push_str(" — second edit");
    }
    println!("After sequential mutations: {}", text);

    // ----------------------------------------------------------
    // CHALLENGE 3: Pass by value — what changes?
    // ----------------------------------------------------------
    println!("\n--- Challenge 3: Pass by value vs by reference ---");

    let original = String::from("Important Document");

    // BY REFERENCE (borrow) — original survives
    let length = get_length_by_ref(&original);
    println!("By ref  → length: {}, original still alive: '{}'", length, original);

    // BY VALUE (ownership move) — original is consumed
    let length = get_length_by_value(original);
    println!("By value → length: {}", length);
    // println!("{}", original); // ❌ ERROR: `original` was moved into the function
    //
    // What changes when you pass by value?
    // • Ownership transfers to the function parameter.
    // • The caller can no longer use the variable.
    // • The function is responsible for the data's lifetime.
    // • Use by-value when the function is meant to *consume* or *own* the data.
    // • Use by-reference when the caller still needs the data afterward.

    // Copy types (i32, f64, bool, char…) are different — they copy automatically:
    let x: i32 = 42;
    let _y = x;          // x is COPIED, not moved
    println!("x is still usable: {}", x); // ✅ Copy trait at work

    // ----------------------------------------------------------
    // CHALLENGE 4: Return &str referencing a local variable
    // ----------------------------------------------------------
    println!("\n--- Challenge 4: Dangling reference ---");

    // The function below does NOT compile. Uncomment to see the error:
    // let greeting = dangling_ref();
    //
    // error[E0106]: missing lifetime specifier
    //   --> src/main.rs:XX:24
    //    |
    //    | fn dangling_ref() -> &str {
    //    |                      ^ expected named lifetime parameter
    //
    // WHY? The local String is dropped when the function returns.
    // Returning a &str to it would be a dangling pointer — Rust refuses.
    //
    // SOLUTION A: Return an owned String (transfer ownership to caller)
    let greeting = safe_greeting_owned();
    println!("Owned greeting: {}", greeting);

    // SOLUTION B: Return a &'static str (a string literal baked into the binary)
    let greeting = safe_greeting_static();
    println!("Static greeting: {}", greeting);

    // SOLUTION C: Borrow from a String that already exists in the caller
    let name = String::from("Alice");
    let greeting = safe_greeting_from_caller(&name);
    println!("Borrowed greeting: {}", greeting);
}

// ============================================================
// CHALLENGE 1 — Mutable reference
// ============================================================

fn append_signature(report: &mut String) {
    // `*report` dereferences to modify the actual String.
    // Rust auto-derefs in most method calls, so .push_str works directly.
    report.push_str("\n-- Signed by: Finance Dept --");
}

// ============================================================
// CHALLENGE 3 — By ref vs by value
// ============================================================

/// Borrows the String → caller keeps ownership
fn get_length_by_ref(s: &String) -> usize {
    s.len() // reading only, no ownership needed
}

/// Takes ownership → String is consumed
fn get_length_by_value(s: String) -> usize {
    let len = s.len();
    len // s is dropped here when the function ends
}

// ============================================================
// CHALLENGE 4 — Dangling reference (DOES NOT COMPILE)
// ============================================================

// ❌ This function would NOT compile — DO NOT uncomment:
//
// fn dangling_ref() -> &str {
//     let local = String::from("I will be dropped!");
//     &local  // ERROR: `local` does not live long enough
//             // It's freed when this function returns, making the
//             // reference point to freed memory — a dangling pointer.
// }

// ✅ SOLUTION A: return ownership
fn safe_greeting_owned() -> String {
    let local = String::from("Hello, owned world!");
    local // ownership moves to caller — no dangling reference
}

// ✅ SOLUTION B: return a string literal (&'static str)
fn safe_greeting_static() -> &'static str {
    "Hello, static world!" // lives for the entire program duration
}

// ✅ SOLUTION C: borrow from data the caller already owns
fn safe_greeting_from_caller(name: &str) -> &str {
    // The returned &str lives as long as `name` does — safe!
    name
}

// ============================================================
// KEY TAKEAWAYS
// ============================================================
//
// Challenge 1 → &mut T lets you modify data in-place without transferring ownership.
//               Use *ref to dereference; Rust auto-derefs for method calls.
//
// Challenge 2 → Only ONE &mut borrow can exist at a time (prevents data races).
//               Scopes ({}) let you reuse a variable mutably in sequence.
//
// Challenge 3 → By value = ownership transfer; caller loses access.
//               By reference = borrow; caller keeps ownership.
//               Copy types (i32, bool…) always copy — no move.
//
// Challenge 4 → Never return a reference to a local variable — it won't compile.
//               Fix: return owned String, &'static str, or borrow from caller's data.