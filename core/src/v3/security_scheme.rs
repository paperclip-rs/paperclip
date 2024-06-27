use super::v2;

macro_rules! to_indexmap {
    ($v2:expr) => {
        $v2.scopes.iter().fold(Default::default(), |mut i, (k, v)| {
            i.insert(k.to_string(), v.to_string());
            i
        })
    };
}

impl From<v2::SecurityScheme> for openapiv3::SecurityScheme {
    fn from(v2: v2::SecurityScheme) -> Self {
        match v2.type_.as_str() {
            "basic" => openapiv3::SecurityScheme::HTTP {
                scheme: "basic".to_string(),
                bearer_format: None,
                description: v2.description,
            },
            "apiKey" => openapiv3::SecurityScheme::APIKey {
                location: match v2.in_.unwrap_or_default().as_str() {
                    "query" => openapiv3::APIKeyLocation::Query,
                    "header" => openapiv3::APIKeyLocation::Header,
                    _ => openapiv3::APIKeyLocation::Query,
                },
                name: v2.name.unwrap_or_default(),
                description: v2.description,
            },
            "oauth2" => {
                let flow = v2.flow.unwrap_or_default();
                openapiv3::SecurityScheme::OAuth2 {
                    flows: openapiv3::OAuth2Flows {
                        implicit: match flow.as_str() {
                            "implicit" => Some(openapiv3::OAuth2Flow::Implicit {
                                authorization_url: v2.auth_url.clone().unwrap_or_default(),
                                refresh_url: None,
                                scopes: to_indexmap!(v2),
                            }),
                            _ => None,
                        },
                        password: match flow.as_str() {
                            "password" => Some(openapiv3::OAuth2Flow::Password {
                                refresh_url: None,
                                token_url: v2.token_url.clone().unwrap_or_default(),
                                scopes: to_indexmap!(v2),
                            }),
                            _ => None,
                        },
                        client_credentials: match flow.as_str() {
                            "application" => Some(openapiv3::OAuth2Flow::ClientCredentials {
                                refresh_url: None,
                                token_url: v2.token_url.clone().unwrap_or_default(),
                                scopes: to_indexmap!(v2),
                            }),
                            _ => None,
                        },
                        authorization_code: match flow.as_str() {
                            "accessCode" => Some(openapiv3::OAuth2Flow::AuthorizationCode {
                                authorization_url: v2.auth_url.clone().unwrap_or_default(),
                                token_url: v2.token_url.clone().unwrap_or_default(),
                                refresh_url: None,
                                scopes: to_indexmap!(v2),
                            }),
                            _ => None,
                        },
                    },
                    description: v2.description,
                }
            }
            type_ => {
                debug_assert!(false, "Invalid Security Type: {}", type_);
                openapiv3::SecurityScheme::HTTP {
                    scheme: "invalid".to_string(),
                    bearer_format: None,
                    description: v2.description,
                }
            }
        }
    }
}
