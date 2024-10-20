use super::{v2, Either};

struct OperationResponse<'a> {
    operation: &'a v2::DefaultOperationRaw,
    response: &'a v2::DefaultResponseRaw,
}

impl From<OperationResponse<'_>> for openapiv3::Response {
    fn from(v2: OperationResponse<'_>) -> Self {
        openapiv3::Response {
            description: v2.response.description.clone().unwrap_or_default(),
            headers: v2
                .response
                .headers
                .iter()
                .fold(Default::default(), |mut i, b| {
                    i.insert(
                        b.0.to_string(),
                        openapiv3::ReferenceOr::Item(b.1.clone().into()),
                    );
                    i
                }),
            content: {
                match v2.response.schema.clone() {
                    Some(response) => {
                        let is_file = v2
                            .response
                            .schema
                            .as_ref()
                            .and_then(|s| s.data_type.map(|d| d == v2::DataType::File))
                            .unwrap_or_default();
                        let media = openapiv3::MediaType {
                            schema: Some(response.into()),
                            ..Default::default()
                        };

                        let mut map = openapiv3::Response::default().content;
                        match v2.operation.produces.as_ref() {
                            Some(range) => {
                                for mime in range {
                                    map.insert(mime.0.to_string(), media.clone());
                                }
                            }
                            None => {
                                if is_file {
                                    // perhaps we should be conservative and use "*/*" instead?
                                    map.insert("multipart/form-data".to_string(), media);
                                } else {
                                    map.insert(v2::SpecFormat::Json.mime().0.to_string(), media);
                                }
                            }
                        }
                        map
                    }
                    None => Default::default(),
                }
            },
            extensions: Default::default(),
            links: Default::default(),
        }
    }
}

pub(crate) struct OperationEitherResponse<'a> {
    pub(crate) operation: &'a v2::DefaultOperationRaw,
    pub(crate) response: &'a Either<v2::Reference, v2::DefaultResponseRaw>,
}

impl From<OperationEitherResponse<'_>> for openapiv3::ReferenceOr<openapiv3::Response> {
    fn from(v2: OperationEitherResponse<'_>) -> Self {
        match v2.response {
            Either::Left(reference) => {
                let response = openapiv3::Response {
                    description: "".to_string(),
                    headers: Default::default(),
                    content: {
                        let media = openapiv3::MediaType {
                            schema: Some(reference.into()),
                            ..Default::default()
                        };
                        let mut map = openapiv3::Response::default().content;
                        match v2.operation.produces.as_ref() {
                            Some(range) => {
                                for mime in range {
                                    map.insert(mime.0.to_string(), media.clone());
                                }
                            }
                            None => {
                                // perhaps we should be conservative and use "*/*" instead?
                                map.insert(v2::SpecFormat::Json.mime().0.to_string(), media);
                            }
                        }
                        map
                    },
                    links: Default::default(),
                    extensions: Default::default(),
                };
                openapiv3::ReferenceOr::Item(response)
            }
            Either::Right(response) => {
                let response = OperationResponse {
                    operation: v2.operation,
                    response,
                };
                openapiv3::ReferenceOr::Item(response.into())
            }
        }
    }
}

impl From<v2::DefaultResponseRaw> for openapiv3::ReferenceOr<openapiv3::Response> {
    fn from(v2: v2::DefaultResponseRaw) -> Self {
        let fake_op = v2::DefaultOperationRaw::default();
        let response = OperationResponse {
            operation: &fake_op,
            response: &v2,
        };
        openapiv3::ReferenceOr::Item(response.into())
    }
}
