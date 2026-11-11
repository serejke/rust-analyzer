# SSR reproduction

This crate reproduces two problems in structural search and replace (SSR).

## Problem 1: the `search` and `ssr` CLI commands panic

The CLI does not attach the database. A pattern that needs a type check panics.

Procedure:

1. Go to this directory:
   ```sh
   cd ssr-repro
   ```
2. Run a search with a pattern that has a qualified path to a method:
   ```sh
   rust-analyzer search 'S::step($s)'
   ```
3. Make sure that the output shows this panic:
   ```text
   Tried to use attached db, but no db is attached
   ```

Expected result: the command prints `s.step()` and `S::step(s)`.

## Problem 2: a template path to a method with a `self` parameter gives no edits

Procedure:

1. Run the test that reproduces the problem:
   ```sh
   cargo test -p ide-ssr -- template_path_to_method_with_self_param
   ```
2. Make sure that the test fails with `No edits were made`.
3. Run the test that uses a method call template:
   ```sh
   cargo test -p ide-ssr -- template_method_call_form_works
   ```
4. Make sure that this test passes.
