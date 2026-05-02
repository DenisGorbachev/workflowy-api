# Workflowy API concepts

## `workflowy-api`

A Rust package with the following crates:

- [`workflowy-api` lib crate](#workflowy-api-lib-crate)
- [`workflowy-api` bin crate](#workflowy-api-bin-crate)

Requirements:

- Must have dependencies:
  - `reqwest`
  - `governor`
  - `subtype`
  - `timestamp-please`
  - `url-macro`
  - `errgonomic`
  - `serde`
- May have dependencies:
  - `strum`

## `workflowy-api` lib crate

A Rust crate that exports the types related to Workflowy API.

Requirements:

- Must contain [request cow types](#request-cow-type) for each request in the API
- Must contain [response types](#response-type) for each response in the API
- Must derive `Serialize` and `Deserialize` from `serde`
  - Should not contain manual `Serialize` and `Deserialize` impls
  - Should use type-level `serde` attributes (e.g. `rename_all`)
- Must have the following layout:
  - Request data types in `request` module
  - Response data types in `response` module
  - Common data types in `common` module
  - Common technical types in

## Key

A type alias for API key as `String`.

## Client

A Rust struct that contains the fields for data that is shared between API requests.

Requirements:

- Must have attributes:
  - `#[derive(From, Into, Eq, PartialEq, Clone, Debug)]`
- Must have fields:
  - `pub inner: HttpClient` (`use reqwest::Client as HttpClient;`)
  - `pub base: Url`
  - `pub limiter: Limiter`
- Must have methods:
  - `pub fn new(key: impl Into<Key>) -> Result<Self, ClientNewError>`
    - Must call `Self::try_from`
- Must have impls:
  - `From<Client>`
  - `TryFrom<Key>`
    - Must set the bearer auth header via `default_headers`

## Limiter

A Rust struct that contains one field per rate limit imposed by the API.

Requirements:

- Every field must have a type exported from `governor`
- Must have impls:
  - `Default`

## `workflowy-api` bin crate

## Request cow type

A type for making an API request whose every field has a `Cow` type.

Requirements:

- Ident must end with "Request".
- Must have derives: `Clone`, `Debug`.
- Should have derives: `Eq`, `PartialEq`, `Hash`.
- Every field must be a `Cow`.
- Every field must have its own lifetime.
  - Lifetime name should be short
  - Lifetime name should match the name of the field (e.g. first letter)

## Request ref type

A type with references to data for making an API request.

Requirements:

- Ident must end with "RequestRef".
- Must have derives: `Clone`, `Copy`, `Debug`.
- Should have derives: `Eq`, `PartialEq`, `Hash`.
- Every field must be an immutable reference (not owned).

## Request own type

A type with owned data for making an API request.

Requirements:

- Ident must end with "RequestOwn".
- Must have derives: `Clone`, `Debug`.
- Should have derives: `Eq`, `PartialEq`, `Hash`.
- Every field must be owned (not a reference).

## Response type

A type with owned data for an API response.

Requirements:

- Ident must end with "Response".
- Must have derives: `Clone`, `Debug`.
- Should have derives: `Eq`, `PartialEq`, `Hash`.
