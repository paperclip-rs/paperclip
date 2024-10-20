use super::v2;

impl From<v2::Info> for openapiv3::Info {
    fn from(v2: v2::Info) -> Self {
        openapiv3::Info {
            title: v2.title,
            description: v2.description,
            terms_of_service: v2.terms_of_service,
            contact: v2.contact.map(|c| c.into()),
            license: v2.license.map(From::from),
            version: v2.version,
            extensions: v2
                .extensions
                .into_iter()
                .fold(Default::default(), |mut i, (k, v)| {
                    i.insert(k, v);
                    i
                }),
        }
    }
}
