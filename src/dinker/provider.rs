use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderDinkerData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_dir: Option<PrimField<String>>,
}

struct ProviderDinker_ {
    data: RefCell<ProviderDinkerData>,
}

pub struct ProviderDinker(Rc<ProviderDinker_>);

impl ProviderDinker {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "dinker", alias)
        } else {
            "dinker".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }

    #[doc= "Set the field `cache_dir`.\nCache intermediate build files (namely FROM images)"]
    pub fn set_cache_dir(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cache_dir = Some(v.into());
        self
    }
}

impl Provider for ProviderDinker_ {
    fn extract_type_tf_id(&self) -> String {
        "dinker".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "andrewbaxter/dinker",
            "version": "0.0.15",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderDinker {}

impl BuildProviderDinker {
    pub fn build(self, stack: &mut Stack) -> ProviderDinker {
        let out = ProviderDinker(Rc::new(ProviderDinker_ { data: RefCell::new(ProviderDinkerData {
            alias: None,
            cache_dir: core::default::Default::default(),
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
