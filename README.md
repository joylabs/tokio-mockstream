# tokio-mockstream [![Build Status](https://travis-ci.org/aatxe/tokio-mockstream.svg?branch=master)](https://travis-ci.org/aatxe/tokio-mockstream) [![Crates.io](https://img.shields.io/crates/v/tokio-mockstream.svg)](https://crates.io/crates/tokio-mockstream) [![Built with Spacemacs](https://cdn.rawgit.com/syl20bnr/spacemacs/442d025779da2f62fc86c2082703697714db6514/assets/spacemacs-badge.svg)](http://spacemacs.org) #

A fake stream for testing network applications backed by buffers.

# Usage

```toml
[dependencies]
tokio-mockstream = "1.1"
```

Next, add this to your crate:

```rust
extern crate tokio_mockstream;

use tokio_mockstream::MockStream;
```

The general idea is to treat `MockStream` as you would `TcpStream`. You can find documentation online at [docs.rs](https://docs.rs/tokio-mockstream/).

## Docker

### Authentication

Docker mounts the `~/.config/gcloud` path from the host in order to authenticate the user.
This is the path where the application default credentials are stored, this will require
enabling credentials through with Cloud SDK command:

`gcloud auth application-default login`

## Cloudsmith

### Configure Cloud Smith for Private Repositories

To pull and push crates from Joy Lab's private repository you'll need an API key from [Cloudsmith](https://cloudsmith.io/user/settings/api/)

Cargo uses git to authenticate; run the following to configure your system:

```
CLOUDSMITH_API_KEY=your-cloudsmith-token
git config --global credential.helper store
echo "https://token:${CLOUDSMITH_API_KEY}@dl.cloudsmith.io" > ~/.git-credentials
```

### Download the Cloudsmith CLI

```
pip install cloudsmith-cli
```

### Publish A Crate

To publish changes as a binary to a public crate use the following steps:

1. Increment the version of your libary in Cargo.toml

2. Create a binary

```
cargo package --allow-dirty
```

3. Push your binary to Cloudsmith

```
cloudsmith push cargo joylabs/crates target/package/rustybigtable-X(VERSION).X.X.crate -k CLOUDSMITH_API_KEY
```

#### Cloudsmith Documentation:

```
https://cloudsmith.io/~joylabs/repos/crates/setup/?auth_method=basic_api_key#formats-cargo
https://blog.cloudsmith.io/2019/05/01/worlds-first-private-cargo-registry/
https://github.com/cloudsmith-io/cloudsmith-examples/tree/master/examples/cargo-cli
```

# License

`tokio-mockstream` is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See LICENSE-APACHE, and LICENSE-MIT for details.

## Docker

### Authentication

Docker mounts the `~/.config/gcloud` path from the host in order to authenticate the user.
This is the path where the application default credentials are stored, this will require
enabling credentials through with Cloud SDK command:

`gcloud auth application-default login`

## Cloudsmith

### Configure Cloud Smith for Private Repositories

To pull and push crates from Joy Lab's private repository you'll need an API key from [Cloudsmith](https://cloudsmith.io/user/settings/api/)

Cargo uses git to authenticate; run the following to configure your system:

```
CLOUDSMITH_API_KEY=your-cloudsmith-token
git config --global credential.helper store
echo "https://token:${CLOUDSMITH_API_KEY}@dl.cloudsmith.io" > ~/.git-credentials
```

### Download the Cloudsmith CLI

```
pip install cloudsmith-cli
```

### Publish A Crate

To publish changes as a binary to a public crate use the following steps:

1. Increment the version of your libary in Cargo.toml

2. Create a binary

```
cargo package --allow-dirty
```

3. Push your binary to Cloudsmith

```
cloudsmith push cargo joylabs/crates target/package/rustybigtable-X(VERSION).X.X.crate -k CLOUDSMITH_API_KEY
```

#### Cloudsmith Documentation:

```
https://cloudsmith.io/~joylabs/repos/crates/setup/?auth_method=basic_api_key#formats-cargo
https://blog.cloudsmith.io/2019/05/01/worlds-first-private-cargo-registry/
https://github.com/cloudsmith-io/cloudsmith-examples/tree/master/examples/cargo-cli
```
