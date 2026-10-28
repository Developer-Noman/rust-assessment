# Assessment Result

Work was done in the `tx-router` crate. No crates were added. `cargo test` passes: 9 tests, 0 failed.

This project is a library. There is no application binary to start. The way to check the work is `cargo test`.

## What was fixed

### `submit` rejects duplicate ids

`submit` ignores a transaction when that id is already waiting in the queue, or when that id was already finalized. Submitting the same id twice leaves `pending_count` at 1.

After `finalize_batch`, those ids stay rejected. A new id still submits normally.

### `build_batch` does not panic

- If the limit is larger than the queue, the batch contains the transactions that are waiting.
- If the queue is empty, or the limit is 0, `build_batch` returns `None` and leaves the queue unchanged.

### Concurrent test

`test_concurrent_submissions` referred to `FileProcessor` and `File`, which are not part of this crate, so the test suite did not compile. It now submits 50 transactions on `TxRouter` at the same time and expects `pending_count` to be 50.

## Tests

| Test | What it checks |
| --- | --- |
| `test_submit_and_count` | One submit leaves one pending transaction. |
| `test_batch_creation` | A batch of 3 from 5 waiting transactions leaves 2 pending. |
| `test_finalize_marks_seen` | Resubmitting a finalized id does not increase the pending count. |
| `test_failed_tracking` | `mark_failed` records one failed id. |
| `test_concurrent_submissions` | 50 concurrent submits all stay pending. |
| `test_build_batch_limit_larger_than_queue` | A limit larger than the queue returns every waiting transaction. |
| `test_build_batch_limit_zero` | A limit of 0 returns `None` and keeps the waiting transaction. |
| `test_duplicate_id_is_ignored` | Submitting the same id twice leaves `pending_count` at 1. |
| `test_resubmit_after_finalize` | A finalized id stays rejected, and a new id submits normally. |

## How to run the tests

From this directory:

```bash
cargo test
```

Run one test by name:

```bash
cargo test test_duplicate_id_is_ignored
cargo test test_build_batch_limit_zero
cargo test test_build_batch_limit_larger_than_queue
cargo test test_resubmit_after_finalize
```

Compile without running tests:

```bash
cargo build
```

`cargo run` does nothing useful here. The crate has no `src/main.rs`.
