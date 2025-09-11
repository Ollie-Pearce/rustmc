A library for high concurrency reads.

This library is named after the 2 (identical) tables that are held internally:
- Active - this is the table that all Readers view. This table will never be
  write locked, so readers never face contention.
- Standby - this is the table that writers mutate. A writer should face minimal
  contention retrieving this table since Readers move to the Active table
  whenever calling `.read()`.

There are 2 ways to use this crate:
1. Direct interaction with `AsLock`/`AsLockHandle`. This is more flexible 
   since users can pass in any struct they want and mutate it however they
   choose. All updates though, will need to be done by passing a function
   instead of via mutable methods (`UpdateTables` trait).
2. Using collections which are built out of the primitives but which provide an
   API similar to `RwLock<T>`; writers can directly call to methods without
   having to provide a mutator function.

There are 2 flavors/modules:
1. Lockless - this variant trades off increased performance against changing the
   API to be less like a `RwLock`. This centers around the `AsLockHandle`, which
   is conceptually similar to `Arc<RwLock>` (requires a separate `AsLockHandle`
   per thread/task).
2. Sync - this centers around using an `AsLock`, which is meant to feel like a
   `RwLock`. The main difference is that you still cannot gain direct write
   access to the underlying table due to the need to keep them identical.

The cost of minimizing contention is:
1. Memory - Internally there are 2 copies of the underlying type the user
   created. This is needed to allow there to always be a table that Readers can
   access out without contention.
2. CPU - The writer must apply all updates twice, once to each table. Lock
   contention for the writer should be less than with a plain RwLock due to
   Readers using the active_table, so it's possible that write times themselves
   will drop.

### Example
Example of the 3 usage patters: build your own wrapper, use prebuilt
collections, and use the primitives. Each of these can be done with both sync
and lockless.
```rust
use std::thread::sleep;
use std::time::Duration;
use std::sync::Arc;

// Create wrapper class so that users can interact with the active_standby
// struct via a RwLock-like interface. See the implementation of the
// collections for more examples.
mod wrapper {
    use active_standby::UpdateTables;

    active_standby::generate_lockless_aslockhandle!(i32);

    struct AddOne {}

    impl<'a> UpdateTables<'a, i32, ()> for AddOne {
        fn apply_first(&mut self, table: &'a mut i32) {
            *table = *table + 1;
        }
        fn apply_second(mut self, table: &mut i32) {
            self.apply_first(table);
        }
    }

    // Client's must implement the mutable interface that they want to
    // offer users. Non mutable functions are automatic via Deref.
    impl<'w> AsLockWriteGuard<'w> {
        pub fn add_one(&mut self) {
            self.guard.update_tables(AddOne {})
        }
    }
}

pub fn run_wrapper() {
    let table = wrapper::AsLockHandle::new(0);
    let table2 = table.clone();

    let handle = std::thread::spawn(move || {
        while *table2.read() != 1 {
            sleep(Duration::from_micros(100));
        }
    });

    table.write().add_one();
    handle.join();
}

// Use a premade collection which wraps `AsLock<Vec<T>>`, to provide an
// interface akin to `RwLock<Vec<T>>`.
pub fn run_collection() {
    use active_standby::sync::collections::AsVec;

    let table = Arc::new(AsVec::default());
    let table2 = Arc::clone(&table);

    let handle = std::thread::spawn(move || {
        while *table2.read() != vec![1] {
            sleep(Duration::from_micros(100));
        }
    });

    table.write().push(1);
    handle.join();
}

// Use the raw AsLock interface to update the underlying data.
pub fn run_primitive() {
    use active_standby::sync::AsLock;

    // If the entries in your table are large, you may want to hold only
    // 1 copy shared by both tables. This is safe so long as you never
    // mutate the shared data; only remove and replace it in the table.
    let table = Arc::new(AsLock::new(vec![Arc::new(1)]));
    let table2 = Arc::clone(&table);

    let handle = std::thread::spawn(move || {
        while *table2.read() != vec![Arc::new(2)] {
            sleep(Duration::from_micros(100));
        }
    });

    table.write().update_tables_closure(|table| {
        // Update the entry in the table, not the shared value behind the
        // Arc.
        table[0] = Arc::new(2);
    });
    handle.join();
}

fn main() {
    run_wrapper();
    run_collection();
    run_primitive();
}
```
## Testing
There are a number of tests that come with active_standby (see
tests/tests_script.sh for examples):

[unittests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

[benchmarks](https://doc.rust-lang.org/unstable-book/library-features/test.html)

[loom](https://crates.io/crates/loom)

[LLVM Sanitizers](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html)

[Miri](https://github.com/rust-lang/miri)

[Rudra](https://github.com/sslab-gatech/Rudra)