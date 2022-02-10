
# Powco API Client Rust

Rust Client for Pow.co HTTP & Websockets APIs

## Authentication

Authentication is done via ecdsa key pairs, where some API calls require small payments of Bitcoin

### Generating Keys

```
let keys = powco::Keys::generate();

println!("identifier: {}", keys.identifier);
println!("secret: {}", keys.secret);

```

### Existing Keys

Any valid Bitcoin private key (not HD) and address will work as secret and identifier

## Making API Calls

### Instantiating Client

```
use std::env;

let keys = powco::Keys {
  identifier: env::var("POWCO_CLIENT_ID"),
  identifier: env::var("POWCO_CLIENT_SECRET"),
}

let client = powco::Client { keys: keys }

if !client.has_valid_keys() {
  panic!("Invalid API Key Pair")
}

```

### Listing Available Jobs

This call does not require payment

```
let jobs: Vec{powco::Job} = client.list_available_jobs();
```

### Getting a Job

This call does require payment. Getting a job will allow your boost miner to
provide work for the job and claim the coins contained therein.

```
let job: powco::Job = client.get_job(&job.uid);

```

