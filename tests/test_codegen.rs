#[macro_use]
extern crate lazy_static;

use paperclip::v2::{
    self,
    codegen::{CrateMeta, DefaultEmitter, Emitter, EmitterState},
    models::{Api, DefaultSchema},
};

use std::fs::File;
use std::io::Read;

lazy_static! {
    static ref ROOT: String = String::from(env!("CARGO_MANIFEST_DIR"));
    static ref SCHEMA: Api<DefaultSchema> = {
        let fd = File::open(ROOT.clone() + "/tests/pet-v2.yaml").expect("file?");
        let raw: Api<DefaultSchema> = v2::from_reader(fd).expect("deserializing spec");
        raw.resolve().expect("resolution")
    };
    static ref CODEGEN: () = {
        let mut state = EmitterState::default();
        state.working_dir = (&*ROOT).into();
        state.working_dir.push("tests/test_pet");
        let mut meta = CrateMeta::default();
        meta.authors = Some(vec!["Me <me@example.com>".into()]);
        state.set_meta(meta);

        let emitter = DefaultEmitter::from(state);
        emitter.generate(&SCHEMA).expect("codegen");
    };
    static ref CLI_CODEGEN: () = {
        let _ = &*CODEGEN;
        let mut state = EmitterState::default();
        state.working_dir = (&*ROOT).into();
        state.working_dir.push("tests/test_pet/cli");
        let mut meta = CrateMeta::default();
        meta.authors = Some(vec!["Me <me@example.com>".into()]);
        meta.is_cli = true;
        state.set_meta(meta);

        let emitter = DefaultEmitter::from(state);
        emitter.generate(&SCHEMA).expect("codegen");
    };
}

fn assert_file_contains_content_at(path: &str, matching_content: &str, index: Option<usize>) {
    let _ = &*CODEGEN;

    let mut contents = String::new();
    let mut fd = File::open(path).expect("missing file");
    fd.read_to_string(&mut contents).expect("reading file");

    let result = contents.find(matching_content);
    if let Some(idx) = index {
        assert_eq!(result, Some(idx));
    } else {
        assert!(result.is_some());
    }
}

#[test]
fn test_lib_creation() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/lib.rs"),
        "
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde;

pub mod category {
    include!(\"./category.rs\");
}

pub mod order {
    include!(\"./order.rs\");
}

pub mod pet {
    include!(\"./pet.rs\");
}

pub mod tag {
    include!(\"./tag.rs\");
}

pub mod client {
    use futures::{Future, future};
",
        Some(0),
    );
}

#[test]
fn test_manifest_creation() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/Cargo.toml"),
        "[package]
name = \"test_pet\"
version = \"0.1.0\"
authors = [\"Me <me@example.com>\"]
edition = \"2018\"

[lib]
path = \"lib.rs\"

[dependencies]
failure = \"0.1\"
futures = \"0.1\"
parking_lot = \"0.8\"
reqwest = \"0.9\"
serde = \"1.0\"
",
        Some(0),
    );
}

#[test]
fn test_overridden_path() {
    // We've specified `host` and `basePath` in our spec, so that should be used in place of the placeholder.
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/lib.rs"),
        "
    impl ApiClient for reqwest::r#async::Client {
        #[inline]
        fn request_builder(&self, method: reqwest::Method, rel_path: &str) -> reqwest::r#async::RequestBuilder {
            let mut u = String::from(\"https://pets.com:8888/api\");
            u.push_str(rel_path.trim_start_matches('/'));
            self.request(method, &u)
        }
",
        Some(1630),
    );
}

#[test]
fn test_header_parameters() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/pet.rs"),
        "
impl<XAuth, Id, Name> PetPostBuilder<XAuth, Id, Name> {
    #[inline]
    pub fn x_auth(mut self, value: impl Into<String>) -> PetPostBuilder<crate::generics::XAuthExists, Id, Name> {
        self.inner.param_x_auth = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn x_pet_id(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_x_pet_id = Some(value.into());
        self
    }
",
        Some(3938),
    );

    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/pet.rs"),
        "
impl crate::client::Sendable for PetPostBuilder<crate::generics::XAuthExists, crate::generics::IdExists, crate::generics::NameExists> {
    type Output = crate::pet::Pet;

    const METHOD: reqwest::Method = reqwest::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        \"/pets\".into()
    }

    fn modify(&self, req: reqwest::r#async::RequestBuilder) -> reqwest::r#async::RequestBuilder {
        let mut req = req;
        req = req.header(\"X-Auth\", self.inner.param_x_auth.as_ref().map(std::string::ToString::to_string).expect(\"missing parameter x_auth?\"));
        if let Some(v) = self.inner.param_x_pet_id.as_ref().map(std::string::ToString::to_string) {
            req = req.header(\"X-Pet-ID\", v);
        }

        req
        .json(&self.inner.body)
    }
}
",
        Some(5411),
    );
}

#[test]
fn test_array_response() {
    // If an operation returns an array of objects, then we bind that
    // operation to that object and do `Sendable<Output<Vec<Object>>>`.
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/pet.rs"),
        "
/// Builder created by [`Pet::list_pets`](./struct.Pet.html#method.list_pets) method for a `GET` operation associated with `Pet`.
#[derive(Debug, Clone)]
pub struct PetGetBuilder;


impl crate::client::Sendable for PetGetBuilder {
    type Output = Vec<Pet>;

    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        \"/pets\".into()
    }
}
",
        Some(2978),
    );
}

#[test]
fn test_operation_with_payload_no_arguments() {
    let _ = &*CLI_CODEGEN;
    // An operation with no arguments should enforce payload if it needs one.
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/cli/app.yaml"),
        "
  - add-pet:
      about: \"Add a new pet to the store\"
      args:
        - payload:
            long: payload
            help: \"Path to payload (schema: Pet) or pass '-' for stdin\"
            takes_value: true
            required: true
",
        None,
    );
}

#[test]
fn test_anonymous_object_definitions() {
    let _ = &*CLI_CODEGEN;
    // An object "can" define objects in its schema without referencing
    // them from known definitions. In that case, we autogenerate stuff.
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/order.rs"),
        "#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Order {
    pub address: Option<OrderAddress>,
    pub id: Option<i64>,
    pub list: Option<Vec<OrderList>>,
}
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct OrderAddress {
    pub code: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub name: Option<String>,
}
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct OrderList {
    #[serde(rename = \"petId\")]
    pub pet_id: Option<i64>,
    pub quantity: Option<i64>,
}

impl Order {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> OrderBuilder {
        OrderBuilder {
            body: Default::default(),
        }
    }
}

impl Into<Order> for OrderBuilder {
    fn into(self) -> Order {
        self.body
    }
}

/// Builder for [`Order`](./struct.Order.html) object.
#[derive(Debug, Clone)]
pub struct OrderBuilder {
    body: self::Order,
}

impl OrderBuilder {
    #[inline]
    pub fn address(mut self, value: impl Into<OrderAddress>) -> Self {
        self.body.address = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn list(mut self, value: impl Iterator<Item = impl Into<OrderList>>) -> Self {
        self.body.list = Some(value.map(|value| value.into()).collect::<Vec<_>>());
        self
    }
}

impl OrderAddress {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> OrderAddressBuilder {
        OrderAddressBuilder {
            body: Default::default(),
        }
    }
}

impl Into<OrderAddress> for OrderAddressBuilder {
    fn into(self) -> OrderAddress {
        self.body
    }
}

/// Builder for [`OrderAddress`](./struct.OrderAddress.html) object.
#[derive(Debug, Clone)]
pub struct OrderAddressBuilder {
    body: self::OrderAddress,
}

impl OrderAddressBuilder {
    #[inline]
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.body.code = Some(value.into());
        self
    }

    #[inline]
    pub fn line1(mut self, value: impl Into<String>) -> Self {
        self.body.line1 = Some(value.into());
        self
    }

    #[inline]
    pub fn line2(mut self, value: impl Into<String>) -> Self {
        self.body.line2 = Some(value.into());
        self
    }

    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.body.name = Some(value.into());
        self
    }
}

impl OrderList {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> OrderListBuilder {
        OrderListBuilder {
            body: Default::default(),
        }
    }
}

impl Into<OrderList> for OrderListBuilder {
    fn into(self) -> OrderList {
        self.body
    }
}

/// Builder for [`OrderList`](./struct.OrderList.html) object.
#[derive(Debug, Clone)]
pub struct OrderListBuilder {
    body: self::OrderList,
}

impl OrderListBuilder {
    #[inline]
    pub fn pet_id(mut self, value: impl Into<i64>) -> Self {
        self.body.pet_id = Some(value.into());
        self
    }

    #[inline]
    pub fn quantity(mut self, value: impl Into<i64>) -> Self {
        self.body.quantity = Some(value.into());
        self
    }
}
",
        Some(0),
    );
}
