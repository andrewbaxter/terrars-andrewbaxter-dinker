#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDinker;

#[derive(Serialize)]
struct ImageData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_env: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clear_env: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cmd: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    dest: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_http: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dirs: std::option::Option<std::vec::Vec<Image_Dirs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entrypoint: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    files: std::vec::Vec<Image_Files>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_hash: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_http: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<std::vec::Vec<Image_Ports>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_signal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_dir: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct Image_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ImageData_>,
}

#[derive(Clone)]
pub struct Image(std::rc::Rc<Image_>);

impl Image {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDinker) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Set the field `add_env`.\nAdd these environment variables when running command in container"]
    pub fn set_add_env(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().add_env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `arch`.\nDefaults to `from` image architecture. Required if `from` omitted."]
    pub fn set_arch(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().arch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `clear_env`.\nUser to use if pushing generated image to remote"]
    pub fn set_clear_env(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().clear_env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cmd`.\nOverridable command parts, concatenated after `entrypoint`"]
    pub fn set_cmd(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().cmd = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dest_host`.\nOverride the docker daemon host when using the `docker-daemon` transport. Takes a URL (like `unix:///var/run/docker.sock`)"]
    pub fn set_dest_host(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dest_host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dest_http`.\nAllow http and unverified SSL"]
    pub fn set_dest_http(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().dest_http = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dest_password`.\nPassword to use if pushing generated image to remote"]
    pub fn set_dest_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dest_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dest_user`.\nUser to use if pushing generated image to remote"]
    pub fn set_dest_user(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dest_user = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dirs`.\nDirs to create in the image. This is mostly to explicitly set the file mode of intermediate directories."]
    pub fn set_dirs(self, v: impl std::convert::Into<std::vec::Vec<Image_Dirs>>) -> Self {
        self.0.data.borrow_mut().dirs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entrypoint`.\nUn-overridable command parts, concatenated before `cmd`"]
    pub fn set_entrypoint(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().entrypoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `from`.\nFROM image to base generated image on; skopeo-style reference, see <https://github.com/containers/image/blob/main/docs/containers-transports.5.md> for a full list. If not specified, has no base layer. This is cached and will only be downloaded once (unless the specifier changes or from_hash is explicitly set to a new value)."]
    pub fn set_from(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().from = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `from_hash`.\nHash representing the contents of the FROM image, to force re-download even if the specifier doesn't change."]
    pub fn set_from_hash(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().from_hash = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `from_host`.\nOverride the docker daemon host when using the `docker-daemon` transport. Takes a URL (like `unix:///var/run/docker.sock`)"]
    pub fn set_from_host(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().from_host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_http`.\nAllow http and unverified SSL"]
    pub fn set_from_http(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().from_http = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_password`.\nPassword to use if pulling FROM image from remote"]
    pub fn set_from_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().from_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_user`.\nUser to use if pulling FROM image from remote"]
    pub fn set_from_user(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().from_user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\nMetadata to attach to image"]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `os`.\nDefaults to `from` image os. Required if `from` omitted."]
    pub fn set_os(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().os = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ports`.\nContainer ports to expose"]
    pub fn set_ports(self, v: impl std::convert::Into<std::vec::Vec<Image_Ports>>) -> Self {
        self.0.data.borrow_mut().ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stop_signal`.\nSignal to use to stop command in container when shutting down"]
    pub fn set_stop_signal(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().stop_signal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\nUser to run command as in container; defaults to user in FROM image"]
    pub fn set_user(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `working_dir`.\nWorking dir for command in container; defaults to working dir in FROM image"]
    pub fn set_working_dir(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().working_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `add_env` after provisioning.\nAdd these environment variables when running command in container"]
    pub fn add_env(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.add_env", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `arch` after provisioning.\nDefaults to `from` image architecture. Required if `from` omitted."]
    pub fn arch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `clear_env` after provisioning.\nUser to use if pushing generated image to remote"]
    pub fn clear_env(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clear_env", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cmd` after provisioning.\nOverridable command parts, concatenated after `entrypoint`"]
    pub fn cmd(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cmd", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dest` after provisioning.\nWhere to send generated image; skopeo-style reference, see <https://github.com/containers/image/blob/main/docs/containers-transports.5.md> for a full list. This is a pattern - you can add the following strings which will be replaced with generated information:\n\n* `{hash}` - A sha256 sum of all the information used to generate the image (note: this should be stable but has no formal specification and is unrelated to the pushed manifest hash).\n\n* `{short_hash}` - The first hex digits of the hash"]
    pub fn dest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dest_host` after provisioning.\nOverride the docker daemon host when using the `docker-daemon` transport. Takes a URL (like `unix:///var/run/docker.sock`)"]
    pub fn dest_host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest_host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dest_http` after provisioning.\nAllow http and unverified SSL"]
    pub fn dest_http(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest_http", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dest_password` after provisioning.\nPassword to use if pushing generated image to remote"]
    pub fn dest_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dest_user` after provisioning.\nUser to use if pushing generated image to remote"]
    pub fn dest_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest_user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dirs` after provisioning.\nDirs to create in the image. This is mostly to explicitly set the file mode of intermediate directories."]
    pub fn dirs(&self) -> terrars::ListRef<Image_DirsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dirs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entrypoint` after provisioning.\nUn-overridable command parts, concatenated before `cmd`"]
    pub fn entrypoint(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `files` after provisioning.\nFiles to add to image"]
    pub fn files(&self) -> terrars::ListRef<Image_FilesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.files", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `files_hash` after provisioning.\nA hash of the input files, before building the image"]
    pub fn files_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.files_hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from` after provisioning.\nFROM image to base generated image on; skopeo-style reference, see <https://github.com/containers/image/blob/main/docs/containers-transports.5.md> for a full list. If not specified, has no base layer. This is cached and will only be downloaded once (unless the specifier changes or from_hash is explicitly set to a new value)."]
    pub fn from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from_hash` after provisioning.\nHash representing the contents of the FROM image, to force re-download even if the specifier doesn't change."]
    pub fn from_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from_host` after provisioning.\nOverride the docker daemon host when using the `docker-daemon` transport. Takes a URL (like `unix:///var/run/docker.sock`)"]
    pub fn from_host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `from_http` after provisioning.\nAllow http and unverified SSL"]
    pub fn from_http(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_http", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from_password` after provisioning.\nPassword to use if pulling FROM image from remote"]
    pub fn from_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from_user` after provisioning.\nUser to use if pulling FROM image from remote"]
    pub fn from_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hash` after provisioning.\nAddressable content hash of the pushed image manifest in a format `algo:hex` like `sha256:0123abcd...`"]
    pub fn hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\nMetadata to attach to image"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `os` after provisioning.\nDefaults to `from` image os. Required if `from` omitted."]
    pub fn os(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\nContainer ports to expose"]
    pub fn ports(&self) -> terrars::ListRef<Image_PortsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rendered_dest` after provisioning.\n`dest` after interpolating generated information."]
    pub fn rendered_dest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rendered_dest", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stop_signal` after provisioning.\nSignal to use to stop command in container when shutting down"]
    pub fn stop_signal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stop_signal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nUser to run command as in container; defaults to user in FROM image"]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `working_dir` after provisioning.\nWorking dir for command in container; defaults to working dir in FROM image"]
    pub fn working_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.extract_ref()))
    }
}

impl terrars::Referable for Image {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Image { }

impl terrars::ToListMappable for Image {
    type O = terrars::ListRef<ImageRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Image_ {
    fn extract_resource_type(&self) -> std::string::String {
        "dinker_image".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImage {
    pub tf_id: std::string::String,
    #[doc =
        "Where to send generated image; skopeo-style reference, see <https://github.com/containers/image/blob/main/docs/containers-transports.5.md> for a full list. This is a pattern - you can add the following strings which will be replaced with generated information:\n\n* `{hash}` - A sha256 sum of all the information used to generate the image (note: this should be stable but has no formal specification and is unrelated to the pushed manifest hash).\n\n* `{short_hash}` - The first hex digits of the hash"]
    pub dest: terrars::PrimField<std::string::String>,
    #[doc = "Files to add to image"]
    pub files: std::vec::Vec<Image_Files>,
}

impl BuildImage {
    pub fn build(self, stack: &mut terrars::Stack) -> Image {
        let out = Image(std::rc::Rc::new(Image_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ImageData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                add_env: core::default::Default::default(),
                arch: core::default::Default::default(),
                clear_env: core::default::Default::default(),
                cmd: core::default::Default::default(),
                dest: self.dest,
                dest_host: core::default::Default::default(),
                dest_http: core::default::Default::default(),
                dest_password: core::default::Default::default(),
                dest_user: core::default::Default::default(),
                dirs: core::default::Default::default(),
                entrypoint: core::default::Default::default(),
                files: self.files,
                from: core::default::Default::default(),
                from_hash: core::default::Default::default(),
                from_host: core::default::Default::default(),
                from_http: core::default::Default::default(),
                from_password: core::default::Default::default(),
                from_user: core::default::Default::default(),
                labels: core::default::Default::default(),
                os: core::default::Default::default(),
                ports: core::default::Default::default(),
                stop_signal: core::default::Default::default(),
                user: core::default::Default::default(),
                working_dir: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ImageRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `add_env` after provisioning.\nAdd these environment variables when running command in container"]
    pub fn add_env(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.add_env", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `arch` after provisioning.\nDefaults to `from` image architecture. Required if `from` omitted."]
    pub fn arch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arch", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `clear_env` after provisioning.\nUser to use if pushing generated image to remote"]
    pub fn clear_env(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.clear_env", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cmd` after provisioning.\nOverridable command parts, concatenated after `entrypoint`"]
    pub fn cmd(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cmd", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dest` after provisioning.\nWhere to send generated image; skopeo-style reference, see <https://github.com/containers/image/blob/main/docs/containers-transports.5.md> for a full list. This is a pattern - you can add the following strings which will be replaced with generated information:\n\n* `{hash}` - A sha256 sum of all the information used to generate the image (note: this should be stable but has no formal specification and is unrelated to the pushed manifest hash).\n\n* `{short_hash}` - The first hex digits of the hash"]
    pub fn dest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dest_host` after provisioning.\nOverride the docker daemon host when using the `docker-daemon` transport. Takes a URL (like `unix:///var/run/docker.sock`)"]
    pub fn dest_host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest_host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dest_http` after provisioning.\nAllow http and unverified SSL"]
    pub fn dest_http(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest_http", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dest_password` after provisioning.\nPassword to use if pushing generated image to remote"]
    pub fn dest_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dest_user` after provisioning.\nUser to use if pushing generated image to remote"]
    pub fn dest_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest_user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dirs` after provisioning.\nDirs to create in the image. This is mostly to explicitly set the file mode of intermediate directories."]
    pub fn dirs(&self) -> terrars::ListRef<Image_DirsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dirs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entrypoint` after provisioning.\nUn-overridable command parts, concatenated before `cmd`"]
    pub fn entrypoint(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `files` after provisioning.\nFiles to add to image"]
    pub fn files(&self) -> terrars::ListRef<Image_FilesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.files", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `files_hash` after provisioning.\nA hash of the input files, before building the image"]
    pub fn files_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.files_hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from` after provisioning.\nFROM image to base generated image on; skopeo-style reference, see <https://github.com/containers/image/blob/main/docs/containers-transports.5.md> for a full list. If not specified, has no base layer. This is cached and will only be downloaded once (unless the specifier changes or from_hash is explicitly set to a new value)."]
    pub fn from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from_hash` after provisioning.\nHash representing the contents of the FROM image, to force re-download even if the specifier doesn't change."]
    pub fn from_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from_host` after provisioning.\nOverride the docker daemon host when using the `docker-daemon` transport. Takes a URL (like `unix:///var/run/docker.sock`)"]
    pub fn from_host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `from_http` after provisioning.\nAllow http and unverified SSL"]
    pub fn from_http(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_http", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from_password` after provisioning.\nPassword to use if pulling FROM image from remote"]
    pub fn from_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from_user` after provisioning.\nUser to use if pulling FROM image from remote"]
    pub fn from_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hash` after provisioning.\nAddressable content hash of the pushed image manifest in a format `algo:hex` like `sha256:0123abcd...`"]
    pub fn hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\nMetadata to attach to image"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `os` after provisioning.\nDefaults to `from` image os. Required if `from` omitted."]
    pub fn os(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\nContainer ports to expose"]
    pub fn ports(&self) -> terrars::ListRef<Image_PortsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rendered_dest` after provisioning.\n`dest` after interpolating generated information."]
    pub fn rendered_dest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rendered_dest", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stop_signal` after provisioning.\nSignal to use to stop command in container when shutting down"]
    pub fn stop_signal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stop_signal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nUser to run command as in container; defaults to user in FROM image"]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `working_dir` after provisioning.\nWorking dir for command in container; defaults to working dir in FROM image"]
    pub fn working_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Image_Dirs {
    dest: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Image_Dirs {
    #[doc = "Set the field `mode`.\nFile mode in octal, defaults to 0755"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Image_Dirs {
    type O = terrars::BlockAssignable<Image_Dirs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImage_Dirs {
    #[doc = "Where to create the dir in the image"]
    pub dest: terrars::PrimField<std::string::String>,
}

impl BuildImage_Dirs {
    pub fn build(self) -> Image_Dirs {
        Image_Dirs {
            dest: self.dest,
            mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Image_DirsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Image_DirsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Image_DirsRef {
        Image_DirsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Image_DirsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dest` after provisioning.\nWhere to create the dir in the image"]
    pub fn dest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\nFile mode in octal, defaults to 0755"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct Image_Files {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    source: terrars::PrimField<std::string::String>,
}

impl Image_Files {
    #[doc =
        "Set the field `dest`.\nWhere to place the file in the image; defaults to filename of source in image root"]
    pub fn set_dest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\nFile mode in octal, defaults to 0644"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Image_Files {
    type O = terrars::BlockAssignable<Image_Files>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImage_Files {
    #[doc = "Local file to include in image"]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildImage_Files {
    pub fn build(self) -> Image_Files {
        Image_Files {
            dest: core::default::Default::default(),
            mode: core::default::Default::default(),
            source: self.source,
        }
    }
}

#[allow(dead_code)]
pub struct Image_FilesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Image_FilesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Image_FilesRef {
        Image_FilesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Image_FilesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dest` after provisioning.\nWhere to place the file in the image; defaults to filename of source in image root"]
    pub fn dest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dest", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\nFile mode in octal, defaults to 0644"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\nLocal file to include in image"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct Image_Ports {
    port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transport: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Image_Ports {
    #[doc = "Set the field `transport`.\nPort protocol (`tcp`), defaults to `tcp`"]
    pub fn set_transport(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.transport = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Image_Ports {
    type O = terrars::BlockAssignable<Image_Ports>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImage_Ports {
    #[doc = "Internal port to make available"]
    pub port: terrars::PrimField<f64>,
}

impl BuildImage_Ports {
    pub fn build(self) -> Image_Ports {
        Image_Ports {
            port: self.port,
            transport: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Image_PortsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Image_PortsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Image_PortsRef {
        Image_PortsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Image_PortsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nInternal port to make available"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transport` after provisioning.\nPort protocol (`tcp`), defaults to `tcp`"]
    pub fn transport(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transport", self.base))
    }
}
