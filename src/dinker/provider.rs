#![allow(non_camel_case_types)]

use serde::Serialize;

#[derive(Serialize)]
struct ProviderDinkerData_ {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_dir: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ProviderDinker_ {
    data: std::cell::RefCell<ProviderDinkerData_>,
}

pub struct ProviderDinker(std::rc::Rc<ProviderDinker_>);

impl ProviderDinker {
    pub fn provider_ref(&self) -> std::string::String {
        let data = self.0.data.borrow();
        if let std::option::Option::Some(alias) = &data.alias {
            format!("{}.{}", "dinker", alias)
        } else {
            "dinker".into()
        }
    }

    pub fn set_alias(self, alias: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(alias.to_string());
        self
    }

    #[doc = "Set the field `cache_dir`.\nCache intermediate build files (namely FROM images)"]
    pub fn set_cache_dir(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cache_dir = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::Provider for ProviderDinker_ {
    fn extract_type_tf_id(&self) -> std::string::String {
        "dinker".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "andrewbaxter/dinker",
            "version": "0.0.24",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderDinker {}

impl BuildProviderDinker {
    pub fn build(self, stack: &mut terrars::Stack) -> ProviderDinker {
        let out =
            ProviderDinker(std::rc::Rc::new(ProviderDinker_ { data: std::cell::RefCell::new(ProviderDinkerData_ {
                alias: std::option::Option::None,
                cache_dir: core::default::Default::default(),
            }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
