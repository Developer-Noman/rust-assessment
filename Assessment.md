
🚀 Technical Assessment 🚀

- Work in the `tx-router` crate. Time is about 3 hours. Run `cargo test` from that directory. Do not add crates.
- `build_batch` must not panic. If the limit is larger than the queue, return the transactions that are waiting. If the queue is empty, or the limit is 0, return `None`.
- `submit` must ignore a transaction whose id is already waiting or already finalized. Submitting the same id twice leaves `pending_count` at 1.
- After `finalize_batch`, those ids stay rejected. A new id still submits normally.
- Add tests for a limit larger than the queue, a limit of 0, a duplicate id, and a resubmit after finalize. `cargo test` must pass.