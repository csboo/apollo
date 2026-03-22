use dioxus::fullstack::HeaderMap;
use dioxus::fullstack::http::header::ACCEPT_LANGUAGE;
use dioxus_i18n::fluent::{FluentArgs, FluentBundle, FluentResource};
use dioxus_i18n::unic_langid::{LanguageIdentifier, langid};

const EN_US: LanguageIdentifier = langid!("en-US");
const HU_HU: LanguageIdentifier = langid!("hu-HU");

const EN_US_FTL: &str = include_str!("../../assets/i18n/en-US.ftl");
const HU_HU_FTL: &str = include_str!("../../assets/i18n/hu-HU.ftl");

pub struct Localizer {
    bundle: FluentBundle<FluentResource>,
}

impl Localizer {
    pub fn from_headers(headers: &HeaderMap) -> Self {
        let preferred = headers
            .get(ACCEPT_LANGUAGE)
            .and_then(|v| v.to_str().ok())
            .and_then(best_supported_language)
            .unwrap_or(HU_HU);
        Self::new(preferred)
    }

    pub fn fallback_hu() -> Self {
        Self::new(HU_HU)
    }

    fn new(selected: LanguageIdentifier) -> Self {
        let selected_is_en = selected.language.as_str() == "en";
        let mut bundle = FluentBundle::new(vec![selected]);

        // Fallback first, selected locale second so selected keys override fallback keys.
        add_resource(&mut bundle, HU_HU_FTL);
        if selected_is_en {
            add_resource(&mut bundle, EN_US_FTL);
        }

        Self { bundle }
    }

    pub fn try_translate_with_args(&self, key: &str, args: Option<&FluentArgs>) -> Option<String> {
        let (message_id, attribute_name) = decompose_identifier(key)?;
        let message = self.bundle.get_message(message_id)?;

        let pattern = if let Some(attribute_name) = attribute_name {
            message.get_attribute(attribute_name)?.value()
        } else {
            message.value()?
        };

        let mut errors = vec![];
        let text = self
            .bundle
            .format_pattern(pattern, args, &mut errors)
            .to_string();
        if errors.is_empty() { Some(text) } else { None }
    }
}

#[macro_export]
macro_rules! s_t {
    ($i18n:expr, $id:expr, $( $name:ident : $value:expr ),* $(,)?) => {{
        let mut params_map = dioxus_i18n::fluent::FluentArgs::new();
        $(
            params_map.set(stringify!($name), $value);
        )*
        ($i18n)
            .try_translate_with_args($id, Some(&params_map))
            .unwrap_or_else(|| panic!("message id not found for key: '{}'", $id))
    }};

    ($i18n:expr, $id:expr $(,)?) => {{
        ($i18n)
            .try_translate_with_args($id, None)
            .unwrap_or_else(|| panic!("message id not found for key: '{}'", $id))
    }};
}

#[macro_export]
macro_rules! s_tid {
    ($i18n:expr, $id:expr, $( $name:ident : $value:expr ),* $(,)?) => {{
        let mut params_map = dioxus_i18n::fluent::FluentArgs::new();
        $(
            params_map.set(stringify!($name), $value);
        )*
        ($i18n)
            .try_translate_with_args($id, Some(&params_map))
            .unwrap_or_else(|| $id.to_string())
    }};

    ($i18n:expr, $id:expr $(,)?) => {{
        ($i18n)
            .try_translate_with_args($id, None)
            .unwrap_or_else(|| $id.to_string())
    }};
}

fn add_resource(bundle: &mut FluentBundle<FluentResource>, ftl: &str) {
    let resource = match FluentResource::try_new(ftl.to_string()) {
        Ok(resource) => resource,
        Err((resource, _errors)) => resource,
    };
    bundle.add_resource_overriding(resource);
}

fn decompose_identifier(key: &str) -> Option<(&str, Option<&str>)> {
    let parts: Vec<&str> = key.split('.').collect();
    match parts.as_slice() {
        [message_id] => Some((message_id, None)),
        [message_id, attribute_name] => Some((message_id, Some(attribute_name))),
        _ => None,
    }
}

fn best_supported_language(raw_header: &str) -> Option<LanguageIdentifier> {
    raw_header
        .split(',')
        .filter_map(|entry| {
            let code = entry.split(';').next()?.trim();
            if code.is_empty() {
                return None;
            }
            LanguageIdentifier::from_bytes(code.as_bytes()).ok()
        })
        .find_map(|lang| match lang.language.as_str() {
            "en" => Some(EN_US),
            "hu" => Some(HU_HU),
            _ => None,
        })
}
