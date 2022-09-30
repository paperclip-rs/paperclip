use super::v2;

impl From<v2::DefaultApiRaw> for openapiv3::OpenAPI {
    fn from(v2: v2::DefaultApiRaw) -> Self {
        let mut spec = openapiv3::OpenAPI {
            openapi: "3.0.0".into(),
            tags: v2.tags.iter().cloned().map(From::from).collect(),
            info: v2.info.clone().into(),
            servers: openapi3_servers(v2.schemes, v2.host, v2.base_path),
            external_docs: v2.external_docs.map(From::from),
            ..Default::default()
        };

        let mut components = openapiv3::Components::default();
        for (name, scheme) in v2.security_definitions {
            components
                .security_schemes
                .insert(name, openapiv3::ReferenceOr::Item(scheme.into()));
        }
        components.responses = v2
            .responses
            .iter()
            .fold(indexmap::IndexMap::new(), |mut i, b| {
                i.insert(b.0.to_string(), b.1.clone().into());
                i
            });
        spec.extensions =
            v2.extensions
                .into_iter()
                .fold(indexmap::IndexMap::new(), |mut i, (k, v)| {
                    i.insert(k, v);
                    i
                });
        spec.paths = openapiv3::Paths {
            paths: v2.paths.iter().fold(indexmap::IndexMap::new(), |mut i, b| {
                i.insert(
                    b.0.to_string(),
                    openapiv3::ReferenceOr::Item(b.1.clone().into()),
                );
                i
            }),
            ..Default::default()
        };

        components.schemas = v2
            .definitions
            .iter()
            .fold(indexmap::IndexMap::new(), |mut i, b| {
                i.insert(b.0.to_string(), b.1.clone().into());
                i
            });
        spec.components = Some(components);

        spec
    }
}

fn openapi3_servers(
    schemes: std::collections::BTreeSet<v2::OperationProtocol>,
    host: Option<String>,
    base: Option<String>,
) -> Vec<openapiv3::Server> {
    if schemes.is_empty() && host.is_none() && base.is_none() {
        vec![]
    } else if let Some(host) = host {
        if !schemes.is_empty() {
            schemes
                .into_iter()
                .map(|scheme| {
                    let scheme_str = match scheme {
                        v2::OperationProtocol::Http => "http",
                        v2::OperationProtocol::Https => "https",
                        v2::OperationProtocol::Ws => "ws",
                        v2::OperationProtocol::Wss => "wss",
                    };
                    openapiv3::Server {
                        url: format!("{}://{}{}", scheme_str, host, base.as_deref().unwrap_or("")),
                        description: None,
                        variables: None,
                        extensions: indexmap::IndexMap::new(),
                    }
                })
                .collect()
        } else {
            vec![openapiv3::Server {
                url: format!("//{}{}", host, base.as_deref().unwrap_or("")),
                description: None,
                variables: None,
                extensions: indexmap::IndexMap::new(),
            }]
        }
    } else {
        vec![openapiv3::Server {
            url: base.unwrap_or_else(|| "/".to_string()),
            description: None,
            variables: None,
            extensions: indexmap::IndexMap::new(),
        }]
    }
}
