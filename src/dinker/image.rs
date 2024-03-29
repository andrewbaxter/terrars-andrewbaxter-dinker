use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDinker;

#[derive(Serialize)]
struct ImageData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    add_env: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clear_env: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmd: Option<ListField<PrimField<String>>>,
    dest: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_http: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dirs: Option<Vec<ImageDirsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entrypoint: Option<ListField<PrimField<String>>>,
    files: Vec<ImageFilesEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_hash: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_host: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_http: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<Vec<ImagePortsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_signal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_dir: Option<PrimField<String>>,
}

struct Image_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ImageData>,
}

#[derive(Clone)]
pub struct Image(Rc<Image_>);

impl Image {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDinker) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `add_env`.\nAdd these environment variables when running command in container"]
    pub fn set_add_env(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().add_env = Some(v.into());
        self
    }

    #[doc= "Set the field `arch`.\nDefaults to `from` image architecture. Required if `from` omitted."]
    pub fn set_arch(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().arch = Some(v.into());
        self
    }

    #[doc= "Set the field `clear_env`.\nUser to use if pushing generated image to remote"]
    pub fn set_clear_env(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().clear_env = Some(v.into());
        self
    }

    #[doc= "Set the field `cmd`.\nOverridable command parts, concatenated after `entrypoint`"]
    pub fn set_cmd(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().cmd = Some(v.into());
        self
    }

    #[doc= "Set the field `dest_host`.\nOverride the docker daemon host when using the `docker-daemon` transport. Takes a URL (like `unix:///var/run/docker.sock`)"]
    pub fn set_dest_host(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dest_host = Some(v.into());
        self
    }

    #[doc= "Set the field `dest_http`.\nAllow http and unverified SSL"]
    pub fn set_dest_http(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().dest_http = Some(v.into());
        self
    }

    #[doc= "Set the field `dest_password`.\nPassword to use if pushing generated image to remote"]
    pub fn set_dest_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dest_password = Some(v.into());
        self
    }

    #[doc= "Set the field `dest_user`.\nUser to use if pushing generated image to remote"]
    pub fn set_dest_user(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dest_user = Some(v.into());
        self
    }

    #[doc= "Set the field `dirs`.\nDirs to create in the image. This is mostly to explicitly set the file mode of intermediate directories."]
    pub fn set_dirs(self, v: impl Into<Vec<ImageDirsEl>>) -> Self {
        self.0.data.borrow_mut().dirs = Some(v.into());
        self
    }

    #[doc= "Set the field `entrypoint`.\nUn-overridable command parts, concatenated before `cmd`"]
    pub fn set_entrypoint(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().entrypoint = Some(v.into());
        self
    }

    #[doc= "Set the field `from`.\nFROM image to base generated image on; skopeo-style reference, see <https://github.com/containers/image/blob/main/docs/containers-transports.5.md> for a full list. If not specified, has no base layer. This is cached and will only be downloaded once (unless the specifier changes or from_hash is explicitly set to a new value)."]
    pub fn set_from(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().from = Some(v.into());
        self
    }

    #[doc= "Set the field `from_hash`.\nHash representing the contents of the FROM image, to force re-download even if the specifier doesn't change."]
    pub fn set_from_hash(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().from_hash = Some(v.into());
        self
    }

    #[doc= "Set the field `from_host`.\nOverride the docker daemon host when using the `docker-daemon` transport. Takes a URL (like `unix:///var/run/docker.sock`)"]
    pub fn set_from_host(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().from_host = Some(v.into());
        self
    }

    #[doc= "Set the field `from_http`.\nAllow http and unverified SSL"]
    pub fn set_from_http(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().from_http = Some(v.into());
        self
    }

    #[doc= "Set the field `from_password`.\nPassword to use if pulling FROM image from remote"]
    pub fn set_from_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().from_password = Some(v.into());
        self
    }

    #[doc= "Set the field `from_user`.\nUser to use if pulling FROM image from remote"]
    pub fn set_from_user(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().from_user = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nMetadata to attach to image"]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `os`.\nDefaults to `from` image os. Required if `from` omitted."]
    pub fn set_os(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().os = Some(v.into());
        self
    }

    #[doc= "Set the field `ports`.\nContainer ports to expose"]
    pub fn set_ports(self, v: impl Into<Vec<ImagePortsEl>>) -> Self {
        self.0.data.borrow_mut().ports = Some(v.into());
        self
    }

    #[doc= "Set the field `stop_signal`.\nSignal to use to stop command in container when shutting down"]
    pub fn set_stop_signal(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().stop_signal = Some(v.into());
        self
    }

    #[doc= "Set the field `user`.\nUser to run command as in container; defaults to user in FROM image"]
    pub fn set_user(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user = Some(v.into());
        self
    }

    #[doc= "Set the field `working_dir`.\nWorking dir for command in container; defaults to working dir in FROM image"]
    pub fn set_working_dir(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().working_dir = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `add_env` after provisioning.\nAdd these environment variables when running command in container"]
    pub fn add_env(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.add_env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arch` after provisioning.\nDefaults to `from` image architecture. Required if `from` omitted."]
    pub fn arch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `clear_env` after provisioning.\nUser to use if pushing generated image to remote"]
    pub fn clear_env(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.clear_env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cmd` after provisioning.\nOverridable command parts, concatenated after `entrypoint`"]
    pub fn cmd(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.cmd", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dest` after provisioning.\nWhere to send generated image; skopeo-style reference, see <https://github.com/containers/image/blob/main/docs/containers-transports.5.md> for a full list. This is a pattern - you can add the following strings which will be replaced with generated information:\n\n* `{hash}` - A sha256 sum of all the information used to generate the image (note: this should be stable but has no formal specification and is unrelated to the pushed manifest hash).\n\n* `{short_hash}` - The first hex digits of the hash"]
    pub fn dest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dest_host` after provisioning.\nOverride the docker daemon host when using the `docker-daemon` transport. Takes a URL (like `unix:///var/run/docker.sock`)"]
    pub fn dest_host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dest_http` after provisioning.\nAllow http and unverified SSL"]
    pub fn dest_http(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_http", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dest_password` after provisioning.\nPassword to use if pushing generated image to remote"]
    pub fn dest_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dest_user` after provisioning.\nUser to use if pushing generated image to remote"]
    pub fn dest_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dirs` after provisioning.\nDirs to create in the image. This is mostly to explicitly set the file mode of intermediate directories."]
    pub fn dirs(&self) -> ListRef<ImageDirsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dirs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `entrypoint` after provisioning.\nUn-overridable command parts, concatenated before `cmd`"]
    pub fn entrypoint(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `files` after provisioning.\nFiles to add to image"]
    pub fn files(&self) -> ListRef<ImageFilesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.files", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `files_hash` after provisioning.\nA hash of the input files, before building the image"]
    pub fn files_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.files_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `from` after provisioning.\nFROM image to base generated image on; skopeo-style reference, see <https://github.com/containers/image/blob/main/docs/containers-transports.5.md> for a full list. If not specified, has no base layer. This is cached and will only be downloaded once (unless the specifier changes or from_hash is explicitly set to a new value)."]
    pub fn from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `from_hash` after provisioning.\nHash representing the contents of the FROM image, to force re-download even if the specifier doesn't change."]
    pub fn from_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `from_host` after provisioning.\nOverride the docker daemon host when using the `docker-daemon` transport. Takes a URL (like `unix:///var/run/docker.sock`)"]
    pub fn from_host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `from_http` after provisioning.\nAllow http and unverified SSL"]
    pub fn from_http(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_http", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `from_password` after provisioning.\nPassword to use if pulling FROM image from remote"]
    pub fn from_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `from_user` after provisioning.\nUser to use if pulling FROM image from remote"]
    pub fn from_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hash` after provisioning.\nAddressable content hash of the pushed image manifest in a format `algo:hex` like `sha256:0123abcd...`"]
    pub fn hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nMetadata to attach to image"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `os` after provisioning.\nDefaults to `from` image os. Required if `from` omitted."]
    pub fn os(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ports` after provisioning.\nContainer ports to expose"]
    pub fn ports(&self) -> ListRef<ImagePortsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rendered_dest` after provisioning.\n`dest` after interpolating generated information."]
    pub fn rendered_dest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rendered_dest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stop_signal` after provisioning.\nSignal to use to stop command in container when shutting down"]
    pub fn stop_signal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stop_signal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\nUser to run command as in container; defaults to user in FROM image"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `working_dir` after provisioning.\nWorking dir for command in container; defaults to working dir in FROM image"]
    pub fn working_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.extract_ref()))
    }
}

impl Referable for Image {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Image { }

impl ToListMappable for Image {
    type O = ListRef<ImageRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Image_ {
    fn extract_resource_type(&self) -> String {
        "dinker_image".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImage {
    pub tf_id: String,
    #[doc= "Where to send generated image; skopeo-style reference, see <https://github.com/containers/image/blob/main/docs/containers-transports.5.md> for a full list. This is a pattern - you can add the following strings which will be replaced with generated information:\n\n* `{hash}` - A sha256 sum of all the information used to generate the image (note: this should be stable but has no formal specification and is unrelated to the pushed manifest hash).\n\n* `{short_hash}` - The first hex digits of the hash"]
    pub dest: PrimField<String>,
    #[doc= "Files to add to image"]
    pub files: Vec<ImageFilesEl>,
}

impl BuildImage {
    pub fn build(self, stack: &mut Stack) -> Image {
        let out = Image(Rc::new(Image_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ImageData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
    shared: StackShared,
    base: String,
}

impl Ref for ImageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ImageRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `add_env` after provisioning.\nAdd these environment variables when running command in container"]
    pub fn add_env(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.add_env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arch` after provisioning.\nDefaults to `from` image architecture. Required if `from` omitted."]
    pub fn arch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arch", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `clear_env` after provisioning.\nUser to use if pushing generated image to remote"]
    pub fn clear_env(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.clear_env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cmd` after provisioning.\nOverridable command parts, concatenated after `entrypoint`"]
    pub fn cmd(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.cmd", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dest` after provisioning.\nWhere to send generated image; skopeo-style reference, see <https://github.com/containers/image/blob/main/docs/containers-transports.5.md> for a full list. This is a pattern - you can add the following strings which will be replaced with generated information:\n\n* `{hash}` - A sha256 sum of all the information used to generate the image (note: this should be stable but has no formal specification and is unrelated to the pushed manifest hash).\n\n* `{short_hash}` - The first hex digits of the hash"]
    pub fn dest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dest_host` after provisioning.\nOverride the docker daemon host when using the `docker-daemon` transport. Takes a URL (like `unix:///var/run/docker.sock`)"]
    pub fn dest_host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dest_http` after provisioning.\nAllow http and unverified SSL"]
    pub fn dest_http(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_http", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dest_password` after provisioning.\nPassword to use if pushing generated image to remote"]
    pub fn dest_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dest_user` after provisioning.\nUser to use if pushing generated image to remote"]
    pub fn dest_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dirs` after provisioning.\nDirs to create in the image. This is mostly to explicitly set the file mode of intermediate directories."]
    pub fn dirs(&self) -> ListRef<ImageDirsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dirs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `entrypoint` after provisioning.\nUn-overridable command parts, concatenated before `cmd`"]
    pub fn entrypoint(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `files` after provisioning.\nFiles to add to image"]
    pub fn files(&self) -> ListRef<ImageFilesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.files", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `files_hash` after provisioning.\nA hash of the input files, before building the image"]
    pub fn files_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.files_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `from` after provisioning.\nFROM image to base generated image on; skopeo-style reference, see <https://github.com/containers/image/blob/main/docs/containers-transports.5.md> for a full list. If not specified, has no base layer. This is cached and will only be downloaded once (unless the specifier changes or from_hash is explicitly set to a new value)."]
    pub fn from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `from_hash` after provisioning.\nHash representing the contents of the FROM image, to force re-download even if the specifier doesn't change."]
    pub fn from_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `from_host` after provisioning.\nOverride the docker daemon host when using the `docker-daemon` transport. Takes a URL (like `unix:///var/run/docker.sock`)"]
    pub fn from_host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `from_http` after provisioning.\nAllow http and unverified SSL"]
    pub fn from_http(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_http", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `from_password` after provisioning.\nPassword to use if pulling FROM image from remote"]
    pub fn from_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `from_user` after provisioning.\nUser to use if pulling FROM image from remote"]
    pub fn from_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hash` after provisioning.\nAddressable content hash of the pushed image manifest in a format `algo:hex` like `sha256:0123abcd...`"]
    pub fn hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nMetadata to attach to image"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `os` after provisioning.\nDefaults to `from` image os. Required if `from` omitted."]
    pub fn os(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ports` after provisioning.\nContainer ports to expose"]
    pub fn ports(&self) -> ListRef<ImagePortsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rendered_dest` after provisioning.\n`dest` after interpolating generated information."]
    pub fn rendered_dest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rendered_dest", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stop_signal` after provisioning.\nSignal to use to stop command in container when shutting down"]
    pub fn stop_signal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stop_signal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\nUser to run command as in container; defaults to user in FROM image"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `working_dir` after provisioning.\nWorking dir for command in container; defaults to working dir in FROM image"]
    pub fn working_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ImageDirsEl {
    dest: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}

impl ImageDirsEl {
    #[doc= "Set the field `mode`.\nFile mode in octal, defaults to 0755"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}

impl ToListMappable for ImageDirsEl {
    type O = BlockAssignable<ImageDirsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImageDirsEl {
    #[doc= "Where to create the dir in the image"]
    pub dest: PrimField<String>,
}

impl BuildImageDirsEl {
    pub fn build(self) -> ImageDirsEl {
        ImageDirsEl {
            dest: self.dest,
            mode: core::default::Default::default(),
        }
    }
}

pub struct ImageDirsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImageDirsElRef {
    fn new(shared: StackShared, base: String) -> ImageDirsElRef {
        ImageDirsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImageDirsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dest` after provisioning.\nWhere to create the dir in the image"]
    pub fn dest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest", self.base))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nFile mode in octal, defaults to 0755"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ImageFilesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dest: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    source: PrimField<String>,
}

impl ImageFilesEl {
    #[doc= "Set the field `dest`.\nWhere to place the file in the image; defaults to filename of source in image root"]
    pub fn set_dest(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dest = Some(v.into());
        self
    }

    #[doc= "Set the field `mode`.\nFile mode in octal, defaults to 0644"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}

impl ToListMappable for ImageFilesEl {
    type O = BlockAssignable<ImageFilesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImageFilesEl {
    #[doc= "Local file to include in image"]
    pub source: PrimField<String>,
}

impl BuildImageFilesEl {
    pub fn build(self) -> ImageFilesEl {
        ImageFilesEl {
            dest: core::default::Default::default(),
            mode: core::default::Default::default(),
            source: self.source,
        }
    }
}

pub struct ImageFilesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImageFilesElRef {
    fn new(shared: StackShared, base: String) -> ImageFilesElRef {
        ImageFilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImageFilesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dest` after provisioning.\nWhere to place the file in the image; defaults to filename of source in image root"]
    pub fn dest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest", self.base))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nFile mode in octal, defaults to 0644"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\nLocal file to include in image"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagePortsEl {
    port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transport: Option<PrimField<String>>,
}

impl ImagePortsEl {
    #[doc= "Set the field `transport`.\nPort protocol (`tcp`), defaults to `tcp`"]
    pub fn set_transport(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transport = Some(v.into());
        self
    }
}

impl ToListMappable for ImagePortsEl {
    type O = BlockAssignable<ImagePortsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagePortsEl {
    #[doc= "Internal port to make available"]
    pub port: PrimField<f64>,
}

impl BuildImagePortsEl {
    pub fn build(self) -> ImagePortsEl {
        ImagePortsEl {
            port: self.port,
            transport: core::default::Default::default(),
        }
    }
}

pub struct ImagePortsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ImagePortsElRef {
    fn new(shared: StackShared, base: String) -> ImagePortsElRef {
        ImagePortsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagePortsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\nInternal port to make available"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc= "Get a reference to the value of field `transport` after provisioning.\nPort protocol (`tcp`), defaults to `tcp`"]
    pub fn transport(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transport", self.base))
    }
}
