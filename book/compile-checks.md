## Compile-time checks

API calls often *require* some parameters. Should we miss those parameters when performing a request, either the client will produce a runtime error or the server will reject our request. On the other hand, paperclip's generated client code uses markers to avoid this problem at compile-time.

For example, in the [previous example](build-script.md), in order to fetch a pet, [`petId` parameter](https://github.com/paperclip-rs/paperclip/blob/fa95b023aaf8b6e396c899a93a9eda6fd791505c/openapi/tests/pet-v2.yaml#L42-L47) is required. Let's change the main function in the above example to fetch a pet without its ID.

```rust
let pet = Pet::get_pet_by_id().send(&client).await?;
```

If we try and compile the program, then we'll get the following error:

```
error[E0599]: no method named `send` found for type
`codegen::pet::PetGetBuilder1<codegen::generics::MissingPetId>`
in the current scope
```

Note that the struct `PetGetBuilder1` has been marked with `MissingPetId`. And, `send` is implemented only when the builder has `PetIdExists` marker.

Hence the fix would be to set the required parameter using the relevant method call (which transforms the builder struct).

```rust
let pet = Pet::get_pet_by_id()
    .id(25)
    .send(&client)
    .await?;
```

... and the code will compile.

The same applies to using API objects (with required fields). For example, the [`addPet` operation](https://github.com/paperclip-rs/paperclip/blob/98a2c053c283ebbbef9b17f7e0ac6ddb0e64f77f/tests/pet-v2.yaml#L125-L148) requires `Pet` object to be present in the HTTP body, but then `Pet` object itself requires [`id` and `name` fields](https://github.com/paperclip-rs/paperclip/blob/98a2c053c283ebbbef9b17f7e0ac6ddb0e64f77f/tests/pet-v2.yaml#L44-L46).

So, if we did this:

```rust
let pet = Pet::add_pet().send(&client).await?;
```

... we'd get an error during compilation:

```
no method named `send` found for type `codegen::pet::PetPostBuilder<
    codegen::generics::MissingId,
    codegen::generics::MissingName
>` in the current scope
```

As we can see, the builder struct has been marked with `MissingId` and `MissingName`, but again `send` is implemented only if the struct had `IdExists` and `NameExists` markers.

Now, we change the code to:

```rust
let pet = Pet::add_pet()
    .id(25)
    .name("Milo")
    .send(&client)
    .await?;
```

... and the code will compile.

Similarly, the types of arguments are also enforced.
