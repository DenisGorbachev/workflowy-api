# Workflowy API concepts

## `workflowy-api`

A Rust package with the following crates:

- [`workflowy-api` lib crate](#workflowy-api-lib-crate)
- [`workflowy-api` bin crate](#workflowy-api-bin-crate)

Requirements:

- Must have dependencies:
  - `reqwest`
  - `governor`
  - `serde` (optional)
  - `rkyv` (optional)

## `workflowy-api` lib crate

A Rust crate that exports the types related to Workflowy API.

Requirements:

- Must contain [request ref types](#request-ref-type) for each request in the API
- Must contain [request own types](#request-own-type) for each request in the API
- Must contain [response types](#response-type) for each response in the API

## Key

A type alias for API key as `String`.

## Client

A Rust struct that contains the fields for data that is shared between API requests.

Requirements:

- Must have attributes:
  - `#[derive(From, Into, Eq, PartialEq, Clone, Debug)]`
- Must have fields:
  - `pub key: Key`
  - `pub limiter: Limiter`
- Must have methods:
  - `pub fn new(key: impl Into<Key>)`
    - Must call `Self::from`
- Must have impls:
  - `From<Key>`

## Limiter

A Rust struct that contains one field per rate limit imposed by the API.

Requirements:

- Every field must have a type exported from `governor`
- Must have impls:
  - `Default`

## `workflowy-api` bin crate

## Request ref type

A type with references to data for making an API request.

Requirements:

- Ident must end with "RequestRef".
- Every field must be an immutable reference (not owned).

## Request own type

A type with owned data for making an API request.

Requirements:

- Ident must end with "RequestOwn".
- Every field must be owned (not a reference).

## Response type

A type with owned data for an API response.

Requirements:

- Ident must end with "Response".
