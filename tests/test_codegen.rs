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
        env_logger::builder()
            .filter(Some("paperclip"), log::LevelFilter::Info)
            .init();
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

pub mod get_shipments_id_response {
    include!(\"./get_shipments_id_response.rs\");
}

pub mod order {
    include!(\"./order.rs\");
}

pub mod pet {
    include!(\"./pet.rs\");
}

pub mod post_shipments_body {
    include!(\"./post_shipments_body.rs\");
}

pub mod status {
    include!(\"./status.rs\");
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
lazy_static = \"1.4\"
log = \"0.4\"
mime = { git = \"https://github.com/hyperium/mime\" }
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
        Some(1944),
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
        Some(3952),
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

    fn modify(&self, req: reqwest::r#async::RequestBuilder) -> Result<reqwest::r#async::RequestBuilder, crate::client::ApiError> {
        let mut req = req;
        req = req.header(\"X-Auth\", self.inner.param_x_auth.as_ref().map(std::string::ToString::to_string).expect(\"missing parameter x_auth?\"));
        if let Some(v) = self.inner.param_x_pet_id.as_ref().map(std::string::ToString::to_string) {
            req = req.header(\"X-Pet-ID\", v);
        }

        Ok(req
        .header(reqwest::header::CONTENT_TYPE, \"application/yaml\")
        .body({
            let mut vec = vec![];
            serde_yaml::to_writer(&mut vec, &self.inner.body)?;
            vec
        }))
    }
}
",
        Some(5439),
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
        Some(2992),
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
fn test_anonymous_object_definition_in_schema() {
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
        self.body.list = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
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

#[test]
fn test_anonymous_object_definition_in_body() {
    let _ = &*CLI_CODEGEN;
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/post_shipments_body.rs"),
        "#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PostShipmentsBody {
    pub address: Option<PostShipmentsBodyAddress>,
    #[serde(rename = \"orderId\")]
    pub order_id: Option<String>,
}
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PostShipmentsBodyAddress {
    pub code: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub name: Option<String>,
}

impl PostShipmentsBody {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PostShipmentsBodyBuilder {
        PostShipmentsBodyBuilder {
            body: Default::default(),
        }
    }

    /// Create shipment for order
    #[inline]
    pub fn post() -> PostShipmentsBodyPostBuilder {
        PostShipmentsBodyPostBuilder {
            body: Default::default(),
        }
    }
}

impl Into<PostShipmentsBody> for PostShipmentsBodyBuilder {
    fn into(self) -> PostShipmentsBody {
        self.body
    }
}

impl Into<PostShipmentsBody> for PostShipmentsBodyPostBuilder {
    fn into(self) -> PostShipmentsBody {
        self.body
    }
}

/// Builder for [`PostShipmentsBody`](./struct.PostShipmentsBody.html) object.
#[derive(Debug, Clone)]
pub struct PostShipmentsBodyBuilder {
    body: self::PostShipmentsBody,
}

impl PostShipmentsBodyBuilder {
    #[inline]
    pub fn address(mut self, value: impl Into<PostShipmentsBodyAddress>) -> Self {
        self.body.address = Some(value.into());
        self
    }

    #[inline]
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.body.order_id = Some(value.into());
        self
    }
}

/// Builder created by [`PostShipmentsBody::post`](./struct.PostShipmentsBody.html#method.post) method for a `POST` operation associated with `PostShipmentsBody`.
#[derive(Debug, Clone)]
pub struct PostShipmentsBodyPostBuilder {
    body: self::PostShipmentsBody,
}

impl PostShipmentsBodyPostBuilder {
    #[inline]
    pub fn address(mut self, value: impl Into<PostShipmentsBodyAddress>) -> Self {
        self.body.address = Some(value.into());
        self
    }

    #[inline]
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.body.order_id = Some(value.into());
        self
    }
}

impl crate::client::Sendable for PostShipmentsBodyPostBuilder {
    type Output = PostShipmentsBody;

    const METHOD: reqwest::Method = reqwest::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        \"/shipments\".into()
    }

    fn modify(&self, req: reqwest::r#async::RequestBuilder) -> Result<reqwest::r#async::RequestBuilder, crate::client::ApiError> {
        Ok(req
        .header(reqwest::header::CONTENT_TYPE, \"application/yaml\")
        .body({
            let mut vec = vec![];
            serde_yaml::to_writer(&mut vec, &self.body)?;
            vec
        }))
    }
}

impl PostShipmentsBodyAddress {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PostShipmentsBodyAddressBuilder {
        PostShipmentsBodyAddressBuilder {
            body: Default::default(),
        }
    }
}

impl Into<PostShipmentsBodyAddress> for PostShipmentsBodyAddressBuilder {
    fn into(self) -> PostShipmentsBodyAddress {
        self.body
    }
}

/// Builder for [`PostShipmentsBodyAddress`](./struct.PostShipmentsBodyAddress.html) object.
#[derive(Debug, Clone)]
pub struct PostShipmentsBodyAddressBuilder {
    body: self::PostShipmentsBodyAddress,
}

impl PostShipmentsBodyAddressBuilder {
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
",
        Some(0),
    );
}

#[test]
fn test_anonymous_object_definition_in_response() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/get_shipments_id_response.rs"),
        "#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct GetShipmentsIdResponse {
    pub address: Option<GetShipmentsIdResponseAddress>,
    #[serde(rename = \"createdOn\")]
    pub created_on: Option<String>,
    #[serde(rename = \"orderId\")]
    pub order_id: Option<String>,
    #[serde(rename = \"shippedOn\")]
    pub shipped_on: Option<String>,
}
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct GetShipmentsIdResponseAddress {
    pub code: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub name: Option<String>,
}

impl GetShipmentsIdResponse {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GetShipmentsIdResponseBuilder {
        GetShipmentsIdResponseBuilder {
            body: Default::default(),
        }
    }

    /// Fetch shipment by ID
    #[inline]
    pub fn get() -> GetShipmentsIdResponseGetBuilder<crate::generics::MissingId> {
        GetShipmentsIdResponseGetBuilder {
            inner: Default::default(),
            _param_id: core::marker::PhantomData,
        }
    }
}

impl Into<GetShipmentsIdResponse> for GetShipmentsIdResponseBuilder {
    fn into(self) -> GetShipmentsIdResponse {
        self.body
    }
}

/// Builder for [`GetShipmentsIdResponse`](./struct.GetShipmentsIdResponse.html) object.
#[derive(Debug, Clone)]
pub struct GetShipmentsIdResponseBuilder {
    body: self::GetShipmentsIdResponse,
}

impl GetShipmentsIdResponseBuilder {
    #[inline]
    pub fn address(mut self, value: impl Into<GetShipmentsIdResponseAddress>) -> Self {
        self.body.address = Some(value.into());
        self
    }

    #[inline]
    pub fn created_on(mut self, value: impl Into<String>) -> Self {
        self.body.created_on = Some(value.into());
        self
    }

    #[inline]
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.body.order_id = Some(value.into());
        self
    }

    #[inline]
    pub fn shipped_on(mut self, value: impl Into<String>) -> Self {
        self.body.shipped_on = Some(value.into());
        self
    }
}

/// Builder created by [`GetShipmentsIdResponse::get`](./struct.GetShipmentsIdResponse.html#method.get) method for a `GET` operation associated with `GetShipmentsIdResponse`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GetShipmentsIdResponseGetBuilder<Id> {
    inner: GetShipmentsIdResponseGetBuilderContainer,
    _param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct GetShipmentsIdResponseGetBuilderContainer {
    param_id: Option<String>,
}

impl<Id> GetShipmentsIdResponseGetBuilder<Id> {
    #[inline]
    pub fn id(mut self, value: impl Into<String>) -> GetShipmentsIdResponseGetBuilder<crate::generics::IdExists> {
        self.inner.param_id = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }
}

impl crate::client::Sendable for GetShipmentsIdResponseGetBuilder<crate::generics::IdExists> {
    type Output = GetShipmentsIdResponse;

    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!(\"/shipments/{id}\", id=self.inner.param_id.as_ref().expect(\"missing parameter id?\")).into()
    }
}

impl GetShipmentsIdResponseAddress {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> GetShipmentsIdResponseAddressBuilder {
        GetShipmentsIdResponseAddressBuilder {
            body: Default::default(),
        }
    }
}

impl Into<GetShipmentsIdResponseAddress> for GetShipmentsIdResponseAddressBuilder {
    fn into(self) -> GetShipmentsIdResponseAddress {
        self.body
    }
}

/// Builder for [`GetShipmentsIdResponseAddress`](./struct.GetShipmentsIdResponseAddress.html) object.
#[derive(Debug, Clone)]
pub struct GetShipmentsIdResponseAddressBuilder {
    body: self::GetShipmentsIdResponseAddress,
}

impl GetShipmentsIdResponseAddressBuilder {
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
",
        Some(0),
    );
}

#[test]
fn test_simple_array_parameter_in_path() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/status.rs"),
        "/// Builder created by [`Status::delete`](./struct.Status.html#method.delete) method for a `DELETE` operation associated with `Status`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct StatusDeleteBuilder<PetId> {
    inner: StatusDeleteBuilderContainer,
    _param_pet_id: core::marker::PhantomData<PetId>,
}

#[derive(Debug, Default, Clone)]
struct StatusDeleteBuilderContainer {
    param_pet_id: Option<crate::util::Delimited<i64, crate::util::Csv>>,
}

impl<PetId> StatusDeleteBuilder<PetId> {
    #[inline]
    pub fn pet_id(mut self, value: impl Iterator<Item = impl Into<i64>>) -> StatusDeleteBuilder<crate::generics::PetIdExists> {
        self.inner.param_pet_id = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        unsafe { std::mem::transmute(self) }
    }
}

impl crate::client::Sendable for StatusDeleteBuilder<crate::generics::PetIdExists> {
    type Output = Status;

    const METHOD: reqwest::Method = reqwest::Method::DELETE;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!(\"/pets/{petId}\", petId=self.inner.param_pet_id.as_ref().expect(\"missing parameter pet_id?\")).into()
    }
}
",
        Some(1189),
    );
}

#[test]
fn test_nested_arrays() {
    let _ = &*CLI_CODEGEN;
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/status.rs"),
        "/// Builder created by [`Status::post_1`](./struct.Status.html#method.post_1) method for a `POST` operation associated with `Status`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct StatusPostBuilder1<Values> {
    inner: StatusPostBuilder1Container,
    _param_values: core::marker::PhantomData<Values>,
}

#[derive(Debug, Default, Clone)]
struct StatusPostBuilder1Container {
    param_values: Option<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<String, crate::util::Pipes>, crate::util::Csv>, crate::util::Ssv>, crate::util::Tsv>>,
    param_x_foobar: Option<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<f64, crate::util::Ssv>, crate::util::Tsv>, crate::util::Csv>, crate::util::Pipes>>,
    param_foo: Option<crate::util::Delimited<crate::util::Delimited<String, crate::util::Csv>, crate::util::Multi>>,
}

impl<Values> StatusPostBuilder1<Values> {
    #[inline]
    pub fn values(mut self, value: impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Into<String>>>>>) -> StatusPostBuilder1<crate::generics::ValuesExists> {
        self.inner.param_values = Some(value.map(|value| value.map(|value| value.map(|value| value.map(|value| value.into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn x_foobar(mut self, value: impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Into<f64>>>>>) -> Self {
        self.inner.param_x_foobar = Some(value.map(|value| value.map(|value| value.map(|value| value.map(|value| value.into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn foo(mut self, value: impl Iterator<Item = impl Iterator<Item = impl Into<String>>>) -> Self {
        self.inner.param_foo = Some(value.map(|value| value.map(|value| value.into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into());
        self
    }
}

impl crate::client::Sendable for StatusPostBuilder1<crate::generics::ValuesExists> {
    type Output = Status;

    const METHOD: reqwest::Method = reqwest::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!(\"/test/parameter/{values}\", values=self.inner.param_values.as_ref().expect(\"missing parameter values?\")).into()
    }

    fn modify(&self, req: reqwest::r#async::RequestBuilder) -> Result<reqwest::r#async::RequestBuilder, crate::client::ApiError> {
        let mut req = req;
        if let Some(v) = self.inner.param_x_foobar.as_ref().map(std::string::ToString::to_string) {
            req = req.header(\"X-foobar\", v);
        }

        Ok(req
        .query({
            &self.inner.param_foo.as_ref().map(|v| {
                v.iter().map(|v| (\"foo\", v.to_string())).collect::<Vec<_>>()
            }).unwrap_or_default()
        }))
    }
}
",
        Some(2349),
    );
}
