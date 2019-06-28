# Paperclip CLI

Install `paperclip` CLI with:

```
cargo install paperclip
```

## Generate client library from CLI

Generate the client library for a sample spec using the following command:

```
paperclip --api v2 -o pet https://raw.githubusercontent.com/wafflespeanut/paperclip/master/openapi/tests/pet-v2.yaml
```

> You can also pass a local path to your spec instead of an URL.

This generates the client library for that spec in `./pet` directory.

## Generate console from CLI

You can also generate a console for your API using the CLI by passing the `--cli` flag.

```
paperclip --api v2 -o pet --cli https://raw.githubusercontent.com/wafflespeanut/paperclip/master/openapi/tests/pet-v2.yaml
```

### Build and run the console

```
cargo build && ./target/debug/pet
```

    pet 0.1.0

    USAGE:
        pet [FLAGS] [OPTIONS] --url <url> <SUBCOMMAND>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
        -v, --verbose    Enable verbose mode.

    OPTIONS:
            --ca-cert <ca-cert>            Path to CA certificate to be added to trust store.
            --client-cert <client-cert>    Path to certificate for TLS client verification.
            --client-key <client-key>      Path to private key for TLS client verification.
            --url <url>                    Base URL for your API.

    SUBCOMMANDS:
        add-pet          Add a new pet to the store
        get-pet-by-id    Find pet by ID
        help             Prints this message or the help of the given subcommand(s)
        list-pets        Fetch list of pets

> The console also supports client verification and setting root CA.

The console maps a subcommand to each operation and assigns arguments to parameters. Similar to compile-time checks in the generated client code, the console checks subcommand arguments at runtime.

Let's try to fetch a pet by its ID without passing the ID.

```
./target/debug/pet --url https://example.com/pets get-pet-by-id
```

    error: The following required arguments were not provided:
        --pet-id <pet-id>

    USAGE:
        pet get-pet-by-id --pet-id <pet-id>

    For more information try --help

And, we get an error.
