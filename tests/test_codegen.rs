#[macro_use]
extern crate lazy_static;

use paperclip::v2::{
    self,
    codegen::{CrateMeta, DefaultEmitter, EmitMode, Emitter, EmitterState},
    models::{DefaultSchema, ResolvableApi},
};

use std::fs::File;
use std::io::Read;

lazy_static! {
    static ref ROOT: String = String::from(env!("CARGO_MANIFEST_DIR"));
    static ref SCHEMA: ResolvableApi<DefaultSchema> = {
        let fd = File::open(ROOT.clone() + "/tests/pet-v2.yaml").expect("file?");
        let raw: ResolvableApi<DefaultSchema> = v2::from_reader(fd).expect("deserializing spec");
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
        meta.mode = EmitMode::Crate;
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
        meta.mode = EmitMode::App;
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

pub mod miscellaneous {
    include!(\"./miscellaneous.rs\");
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

pub mod test_enum {
    include!(\"./test_enum.rs\");
}

pub mod test_nested_array_with_object {
    include!(\"./test_nested_array_with_object.rs\");
}

pub mod client {
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
async-trait = \"0.1\"
failure = \"0.1\"
futures = \"0.1\"
futures-preview = { version = \"0.3.0-alpha.19\", features = [\"compat\"], package = \"futures-preview\" }
http = \"0.1\"
lazy_static = \"1.4\"
log = \"0.4\"
mime = { git = \"https://github.com/hyperium/mime\" }
mime_guess = \"2.0\"
parking_lot = \"0.8\"
reqwest = \"0.9\"
serde = \"1.0\"
serde_json = \"1.0\"
serde_yaml = \"0.8\"
tokio-io-old = { version = \"0.1\", package = \"tokio-io\" }
tokio-fs-old = { version = \"0.1\", package = \"tokio-fs\" }
url = \"2.1\"

[workspace]
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
    #[async_trait::async_trait]
    impl ApiClient for reqwest::r#async::Client {
        type Request = reqwest::r#async::RequestBuilder;
        type Response = reqwest::r#async::Response;

        fn request_builder(&self, method: http::Method, rel_path: &str) -> Self::Request {
            let mut u = String::from(\"https://pets.com:8888/api\");
            u.push_str(rel_path.trim_start_matches('/'));
            self.request(method, &u)
        }

        async fn make_request(&self, req: Self::Request) -> Result<Self::Response, ApiError<Self::Response>> {
            let req = req.build().map_err(ApiError::Reqwest)?;
            let resp = self.execute(req).map_err(ApiError::Reqwest).compat().await?;
            Ok(resp)
        }
    }
",
        Some(7261),
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
        Some(4005),
    );

    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/pet.rs"),
        "
impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PetPostBuilder<crate::generics::XAuthExists, crate::generics::IdExists, crate::generics::NameExists> {
    type Output = crate::pet::Pet;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        \"/pets\".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        let mut req = req;
        req = req.header(\"X-Auth\", &self.inner.param_x_auth.as_ref().map(std::string::ToString::to_string).expect(\"missing parameter x_auth?\"));
        if let Some(v) = &self.inner.param_x_pet_id.as_ref().map(std::string::ToString::to_string) {
            req = req.header(\"X-Pet-ID\", &v);
        }

        Ok(req
        .header(http::header::CONTENT_TYPE.as_str(), \"application/yaml\")
        .body_bytes({
            let mut vec = vec![];
            serde_yaml::to_writer(&mut vec, &self.inner.body)?;
            vec
        }))
    }
}
",
        Some(5492),
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


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PetGetBuilder {
    type Output = Vec<Pet>;

    const METHOD: http::Method = http::Method::GET;

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
        "#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Order {
    pub address: Option<crate::order::OrderAddress>,
    pub id: Option<i64>,
    pub list: Option<Vec<crate::order::OrderListItem>>,
    pub status: Option<crate::order::OrderStatus>,
    #[serde(rename = \"test-string-enum\")]
    pub test_string_enum: Option<crate::order::OrderTestStringEnum>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OrderAddress {
    pub code: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub name: Option<String>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OrderListItem {
    #[serde(rename = \"petId\")]
    pub pet_id: Option<i64>,
    pub quantity: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum OrderStatus {
    #[serde(rename = \"paymentPending\")]
    PaymentPending,
    #[serde(rename = \"orderPlaced\")]
    OrderPlaced,
    #[serde(rename = \"shipped\")]
    Shipped,
    #[serde(rename = \"fulfilled\")]
    Fulfilled,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum OrderTestStringEnum {
    #[serde(rename = \"booya\")]
    Booya,
    #[serde(rename = \"72.9\")]
    Number_72_9,
    #[serde(rename = \"true\")]
    True,
    #[serde(rename = \"-53\")]
    Number__53,
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
    pub fn address(mut self, value: crate::order::OrderAddress) -> Self {
        self.body.address = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<i64>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn list(mut self, value: impl Iterator<Item = crate::order::OrderListItem>) -> Self {
        self.body.list = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn status(mut self, value: crate::order::OrderStatus) -> Self {
        self.body.status = Some(value.into());
        self
    }

    #[inline]
    pub fn test_string_enum(mut self, value: crate::order::OrderTestStringEnum) -> Self {
        self.body.test_string_enum = Some(value.into());
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

impl OrderListItem {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> OrderListItemBuilder {
        OrderListItemBuilder {
            body: Default::default(),
        }
    }
}

impl Into<OrderListItem> for OrderListItemBuilder {
    fn into(self) -> OrderListItem {
        self.body
    }
}

/// Builder for [`OrderListItem`](./struct.OrderListItem.html) object.
#[derive(Debug, Clone)]
pub struct OrderListItemBuilder {
    body: self::OrderListItem,
}

impl OrderListItemBuilder {
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
        "#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PostShipmentsBody {
    pub address: Option<crate::post_shipments_body::PostShipmentsBodyAddress>,
    #[serde(rename = \"orderId\")]
    pub order_id: Option<String>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
    #[deprecated]
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
    pub fn address(mut self, value: crate::post_shipments_body::PostShipmentsBodyAddress) -> Self {
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
    pub fn address(mut self, value: crate::post_shipments_body::PostShipmentsBodyAddress) -> Self {
        self.body.address = Some(value.into());
        self
    }

    #[inline]
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.body.order_id = Some(value.into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for PostShipmentsBodyPostBuilder {
    type Output = serde_yaml::Value;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        \"/shipments\".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .header(http::header::CONTENT_TYPE.as_str(), \"application/yaml\")
        .body_bytes({
            let mut vec = vec![];
            serde_yaml::to_writer(&mut vec, &self.body)?;
            vec
        })
        .header(http::header::ACCEPT.as_str(), \"application/yaml\"))
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
        "#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetShipmentsIdResponse {
    pub address: Option<crate::get_shipments_id_response::GetShipmentsIdResponseAddress>,
    #[serde(rename = \"createdOn\")]
    pub created_on: Option<String>,
    #[serde(rename = \"orderId\")]
    pub order_id: Option<String>,
    #[serde(rename = \"shippedOn\")]
    pub shipped_on: Option<String>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
    pub fn get_shipment() -> GetShipmentsIdResponseGetBuilder<crate::generics::MissingId> {
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
    pub fn address(mut self, value: crate::get_shipments_id_response::GetShipmentsIdResponseAddress) -> Self {
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

/// Builder created by [`GetShipmentsIdResponse::get_shipment`](./struct.GetShipmentsIdResponse.html#method.get_shipment) method for a `GET` operation associated with `GetShipmentsIdResponse`.
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for GetShipmentsIdResponseGetBuilder<crate::generics::IdExists> {
    type Output = GetShipmentsIdResponse;

    const METHOD: http::Method = http::Method::GET;

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
        "
/// Builder created by [`Status::delete`](./struct.Status.html#method.delete) method for a `DELETE` operation associated with `Status`.
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

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for StatusDeleteBuilder<crate::generics::PetIdExists> {
    type Output = Status;

    const METHOD: http::Method = http::Method::DELETE;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!(\"/pets/{petId}\", petId=self.inner.param_pet_id.as_ref().expect(\"missing parameter pet_id?\")).into()
    }
}
",
        Some(1285),
    );
}

#[test]
fn test_nested_arrays() {
    // It's in miscellaneous because there's no body parameter or response body.
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/miscellaneous.rs"),
        "
/// Builder created by [`Miscellaneous::get`](./struct.Miscellaneous.html#method.get) method for a `GET` operation associated with `Miscellaneous`.
#[derive(Debug, Clone)]
pub struct MiscellaneousGetBuilder;


impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MiscellaneousGetBuilder {
    type Output = Vec<Vec<crate::test_nested_array_with_object::TestNestedArrayWithObjectItemItem>>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        \"/test/array\".into()
    }
}
",
        Some(626),
    );

    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/miscellaneous.rs"),
        "
/// Builder created by [`Miscellaneous::post_2`](./struct.Miscellaneous.html#method.post_2) method for a `POST` operation associated with `Miscellaneous`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MiscellaneousPostBuilder2<Values> {
    inner: MiscellaneousPostBuilder2Container,
    _param_values: core::marker::PhantomData<Values>,
}

#[derive(Debug, Default, Clone)]
struct MiscellaneousPostBuilder2Container {
    param_values: Option<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<String, crate::util::Pipes>, crate::util::Csv>, crate::util::Ssv>, crate::util::Tsv>>,
    param_x_foobar: Option<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<f64, crate::util::Ssv>, crate::util::Tsv>, crate::util::Csv>, crate::util::Pipes>>,
    param_booya: Option<crate::util::Delimited<crate::util::Delimited<i64, crate::util::Csv>, crate::util::Multi>>,
    param_foo: Option<crate::util::Delimited<crate::util::Delimited<String, crate::util::Csv>, crate::util::Multi>>,
}

impl<Values> MiscellaneousPostBuilder2<Values> {
    #[inline]
    pub fn values(mut self, value: impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Into<String>>>>>) -> MiscellaneousPostBuilder2<crate::generics::ValuesExists> {
        self.inner.param_values = Some(value.map(|value| value.map(|value| value.map(|value| value.map(|value| value.into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn x_foobar(mut self, value: impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Into<f64>>>>>) -> Self {
        self.inner.param_x_foobar = Some(value.map(|value| value.map(|value| value.map(|value| value.map(|value| value.into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn booya(mut self, value: impl Iterator<Item = impl Iterator<Item = impl Into<i64>>>) -> Self {
        self.inner.param_booya = Some(value.map(|value| value.map(|value| value.into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn foo(mut self, value: impl Iterator<Item = impl Iterator<Item = impl Into<String>>>) -> Self {
        self.inner.param_foo = Some(value.map(|value| value.map(|value| value.into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MiscellaneousPostBuilder2<crate::generics::ValuesExists> {
    type Output = String;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!(\"/test/parameter/{values}\", values=self.inner.param_values.as_ref().expect(\"missing parameter values?\")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        let mut req = req;
        if let Some(v) = &self.inner.param_x_foobar.as_ref().map(std::string::ToString::to_string) {
            req = req.header(\"X-foobar\", &v);
        }

        Ok(req
        .body_bytes({
            let mut ser = url::form_urlencoded::Serializer::new(String::new());
            if let Some(stuff) = self.inner.param_booya.as_ref() {
                for v in stuff.iter() {
                    ser.append_pair(\"booya\", &v.to_string());
                }
            }
            ser.finish().into_bytes()
        })
        .header(http::header::CONTENT_TYPE.as_str(), \"application/x-www-form-urlencoded\")
        .query({
            &self.inner.param_foo.as_ref().map(|v| {
                v.iter().map(|v| (\"foo\", v.to_string())).collect::<Vec<_>>()
            }).unwrap_or_default()
        }))
    }
}
",
        Some(2085),
    );
}

#[test]
fn test_builder_from_args_with_delimited() {
    let _ = &*CLI_CODEGEN;

    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/cli/miscellaneous.rs"),
        "
#[allow(unused_variables)]
impl MiscellaneousPostBuilder2<crate::generics::ValuesExists> {
    pub(crate) fn from_args(matches: Option<&clap::ArgMatches<'_>>) -> Result<Self, crate::ClientError> {
        let thing = MiscellaneousPostBuilder2 {
            inner: MiscellaneousPostBuilder2Container {
            param_values: matches.and_then(|m| {
                    m.value_of(\"values\").map(|_| {
                        value_t!(m, \"values\", crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<String, crate::util::Pipes>, crate::util::Csv>, crate::util::Ssv>, crate::util::Tsv>).unwrap_or_else(|e| e.exit())
                    })
                }),

            param_x_foobar: matches.and_then(|m| {
                    m.value_of(\"x-foobar\").map(|_| {
                        value_t!(m, \"x-foobar\", crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<crate::util::Delimited<f64, crate::util::Ssv>, crate::util::Tsv>, crate::util::Csv>, crate::util::Pipes>).unwrap_or_else(|e| e.exit())
                    })
                }),

            param_booya: matches.and_then(|m| {
                    m.value_of(\"booya\").map(|_| {
                        value_t!(m, \"booya\", crate::util::Delimited<crate::util::Delimited<i64, crate::util::Csv>, crate::util::Multi>).unwrap_or_else(|e| e.exit())
                    })
                }),

            param_foo: matches.and_then(|m| {
                    m.value_of(\"foo\").map(|_| {
                        value_t!(m, \"foo\", crate::util::Delimited<crate::util::Delimited<String, crate::util::Csv>, crate::util::Multi>).unwrap_or_else(|e| e.exit())
                    })
                }),

            },
            _param_values: core::marker::PhantomData,
        };

        Ok(thing)
    }
}
",
        Some(6627),
    );
}

#[test]
fn test_file_response() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/miscellaneous.rs"),
        "
/// Builder created by [`Miscellaneous::get_1`](./struct.Miscellaneous.html#method.get_1) method for a `GET` operation associated with `Miscellaneous`.
#[derive(Debug, Clone)]
pub struct MiscellaneousGetBuilder1;


#[async_trait::async_trait]
impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for MiscellaneousGetBuilder1 {
    type Output = crate::util::ResponseStream<<<Client as crate::client::ApiClient>::Response as crate::client::Response>::Stream>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        \"/test/file\".into()
    }

    async fn send(&self, client: &Client) -> Result<Self::Output, crate::client::ApiError<Client::Response>> {
        use crate::client::Response;

        let mut resp = self.send_raw(client).await?;
        Ok(resp.stream())
    }
}
",
        Some(1206),
    );
}

#[test]
fn test_multipart_with_file() {
    let _ = &*CLI_CODEGEN;

    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/cli/status.rs"),
        "
/// Builder created by [`Status::put_1`](./struct.Status.html#method.put_1) method for a `PUT` operation associated with `Status`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct StatusPutBuilder1<SomeDataFile, Foobar> {
    inner: StatusPutBuilder1Container,
    _param_some_data_file: core::marker::PhantomData<SomeDataFile>,
    _param_foobar: core::marker::PhantomData<Foobar>,
}

#[derive(Debug, Default, Clone)]
struct StatusPutBuilder1Container {
    param_some_data_file: Option<std::path::PathBuf>,
    param_some_other_file: Option<std::path::PathBuf>,
    param_foobar: Option<String>,
    param_booya: Option<crate::util::Delimited<crate::util::Delimited<i64, crate::util::Csv>, crate::util::Multi>>,
}

impl<SomeDataFile, Foobar> StatusPutBuilder1<SomeDataFile, Foobar> {
    #[inline]
    pub fn some_data_file(mut self, value: impl AsRef<std::path::Path>) -> StatusPutBuilder1<crate::generics::SomeDataFileExists, Foobar> {
        self.inner.param_some_data_file = Some(value.as_ref().into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn some_other_file(mut self, value: impl AsRef<std::path::Path>) -> Self {
        self.inner.param_some_other_file = Some(value.as_ref().into());
        self
    }

    #[inline]
    pub fn foobar(mut self, value: impl Into<String>) -> StatusPutBuilder1<SomeDataFile, crate::generics::FoobarExists> {
        self.inner.param_foobar = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn booya(mut self, value: impl Iterator<Item = impl Iterator<Item = impl Into<i64>>>) -> Self {
        self.inner.param_booya = Some(value.map(|value| value.map(|value| value.into()).collect::<Vec<_>>().into()).collect::<Vec<_>>().into());
        self
    }
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client> for StatusPutBuilder1<crate::generics::SomeDataFileExists, crate::generics::FoobarExists> {
    type Output = Status;

    const METHOD: http::Method = http::Method::PUT;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        \"/test/file\".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
        use crate::client::Request;
        Ok(req
        .multipart_form_data({
            use crate::client::Form;
            let mut form = <Client::Request as Request>::Form::new();
            if let Some(v) = self.inner.param_some_data_file.as_ref() {
                form = form.file(\"someDataFile\", v)?;
            }
            if let Some(v) = self.inner.param_some_other_file.as_ref() {
                form = form.file(\"someOtherFile\", v)?;
            }
            if let Some(v) = self.inner.param_foobar.as_ref() {
                form = form.text(\"foobar\", v.to_string());
            }
            if let Some(stuff) = self.inner.param_booya.as_ref() {
                for v in stuff.iter() {
                    form = form.text(\"booya\", v.to_string());
                }
            }
            form
        }))
    }
}

#[allow(unused_variables)]
impl StatusPutBuilder1<crate::generics::SomeDataFileExists, crate::generics::FoobarExists> {
    pub(crate) fn from_args(matches: Option<&clap::ArgMatches<'_>>) -> Result<Self, crate::ClientError> {
        let thing = StatusPutBuilder1 {
            inner: StatusPutBuilder1Container {
            param_some_data_file: matches.and_then(|m| {
                    m.value_of(\"some-data-file\").map(|_| {
                        value_t!(m, \"some-data-file\", std::path::PathBuf).unwrap_or_else(|e| e.exit())
                    })
                }),

            param_some_other_file: matches.and_then(|m| {
                    m.value_of(\"some-other-file\").map(|_| {
                        value_t!(m, \"some-other-file\", std::path::PathBuf).unwrap_or_else(|e| e.exit())
                    })
                }),

            param_foobar: matches.and_then(|m| {
                    m.value_of(\"foobar\").map(|_| {
                        value_t!(m, \"foobar\", String).unwrap_or_else(|e| e.exit())
                    })
                }),

            param_booya: matches.and_then(|m| {
                    m.value_of(\"booya\").map(|_| {
                        value_t!(m, \"booya\", crate::util::Delimited<crate::util::Delimited<i64, crate::util::Csv>, crate::util::Multi>).unwrap_or_else(|e| e.exit())
                    })
                }),

            },
            _param_some_data_file: core::marker::PhantomData,
            _param_foobar: core::marker::PhantomData,
        };

        Ok(thing)
    }
}
",
        Some(3154),
    );
}

#[test]
fn test_simple_any_enum() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_pet/test_enum.rs"),
        "#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum TestEnum {
    True,
    Number_1_5,
    Number_23,
    Number_964,
    Number__79_23,
    Number_14343,
    Number__964,
    Hello,
    Foo,
    Bar,
}
impl serde::Serialize for TestEnum {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        match self {
            TestEnum::True => (true).serialize(ser),
            TestEnum::Number_1_5 => (1.5).serialize(ser),
            TestEnum::Number_23 => (23).serialize(ser),
            TestEnum::Number_964 => (964).serialize(ser),
            TestEnum::Number__79_23 => (-79.23).serialize(ser),
            TestEnum::Number_14343 => (14343).serialize(ser),
            TestEnum::Number__964 => (-964).serialize(ser),
            TestEnum::Hello => (\"hello\").serialize(ser),
            TestEnum::Foo => (\"foo\").serialize(ser),
            TestEnum::Bar => (\"bar\").serialize(ser),
        }
    }
}
impl<'de> serde::Deserialize<'de> for TestEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deser: D) -> Result<Self, D::Error> {
        use serde::de::{Error, Unexpected, Visitor};
        struct VariantVisitor;
        const EXPECT_MSG: &str = \"valid value for enum TestEnum\";

        impl<'de> Visitor<'de> for VariantVisitor {
            type Value = TestEnum;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(EXPECT_MSG)
            }

            fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
                if v == true {
                    return Ok(TestEnum::True);
                }
                Err(E::invalid_value(Unexpected::Bool(v), &EXPECT_MSG))
            }

            fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
                if v == -964 {
                    return Ok(TestEnum::Number__964);
                }
                Err(E::invalid_value(Unexpected::Signed(v), &EXPECT_MSG))
            }

            fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
                if v == 23 {
                    return Ok(TestEnum::Number_23);
                }
                if v == 964 {
                    return Ok(TestEnum::Number_964);
                }
                if v == 14343 {
                    return Ok(TestEnum::Number_14343);
                }
                Err(E::invalid_value(Unexpected::Unsigned(v), &EXPECT_MSG))
            }

            fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
                if v == 1.5 {
                    return Ok(TestEnum::Number_1_5);
                }
                if v == -79.23 {
                    return Ok(TestEnum::Number__79_23);
                }
                Err(E::invalid_value(Unexpected::Float(v), &EXPECT_MSG))
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                if v == \"hello\" {
                    return Ok(TestEnum::Hello);
                }
                if v == \"foo\" {
                    return Ok(TestEnum::Foo);
                }
                if v == \"bar\" {
                    return Ok(TestEnum::Bar);
                }
                Err(E::invalid_value(Unexpected::Str(v), &EXPECT_MSG))
            }
        }

        deser.deserialize_any(VariantVisitor)
    }
}
",
        Some(0),
    );
}
