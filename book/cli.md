# Paperclip CLI

Install `paperclip` CLI with:

```
cargo install paperclip --features cli
```

## Generate client library from CLI

You can generate the client library for some spec using the following command:

```
wget https://raw.githubusercontent.com/paperclip-rs/paperclip/master/tests/pet-v2.yaml
paperclip --api v2 -o pet pet-v2.yaml
```

This generates the client library for that spec in `./pet` directory.

## Generate console from CLI

You can also generate a console for your API using the CLI by passing the `--cli` flag.

```
wget https://raw.githubusercontent.com/paperclip-rs/paperclip/master/tests/pet-v2.yaml
paperclip --api v2 -o pet --cli pet-v2.yaml
```

### Build and run the console

> **NOTE:** I'm using the debug build throughout this example.

```
cd pet && cargo build && ./target/debug/pet
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

Note that passing **`--url` is mandatory**. It's the base URL for your API.

> The console also supports client verification and setting root CA.

## Runtime checks

The generated console associates subcommands to operations and arguments to parameters. Similar to compile-time checks in the generated client code, the console checks subcommand arguments at runtime.

#### Parameters as arguments

Let's try to fetch a pet by its ID without passing the ID.

```
./target/debug/pet --url https://example.com/pets get-pet-by-id
```

    error: The following required arguments were not provided:
        --pet-id <pet-id>

    USAGE:
        pet get-pet-by-id --pet-id <pet-id>

    For more information try --help

... and we get an error.

Now, let's add that argument with a valid value (i.e., value that can be parsed to the expected type, which in this case, is an integer).

```
./target/debug/pet -v --url http://echo.jsontest.com get-pet-by-id get-pet-by-id --pet-id 25

GET http://echo.jsontest.com/pets/25
200 OK
{"pets": "25"}
```

Passing `-v` flag enables verbose mode which prints additional information about the request we've made. Also note that the body of the response is piped to stdout directly.

#### Request body

Finally, let's `POST` something with a body. Here, `add-pet` requires a payload, so `--payload` argument is required. This argument is special in that it could be either a path to a file or `-` (when input is obtained from stdin). Either way, the input is parsed to the actual schema before making the API call.

```
./target/debug/pet -v --url http://localhost:8000 add-pet --payload - << EOF
{"id": 25, "name": "Milo"}
EOF

POST http://localhost:8000/pets
200 OK
{"status": "ok"}
```
