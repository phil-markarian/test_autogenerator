
# Automated Test Generator for cypress

Automatically generates tests and their appropriate test folders using a CSV containing 1 to N websites.

This program is used to autu-generate tests for cypress.

## How to use

`cargo build --release`

You will need to place your CSV file in your `release/target` folder.

Change your local development server in these places

```rust
let mut url = Url::parse("http://localhost:8080").unwrap();
```

```rust
let local_site = format!("http://localhost:8080{}", page_url.path());
```

To run the program you will do the following steps.

`./release/target/test_generator YourCSVFileName.csv 1 1 1000`

The first input is your CSV file name. 
The second input is the column you wish to specify.
The following two inputs are the rows you wish to make tests for.

Your generated tests will be placed in '/target/release/cypress'