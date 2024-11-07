// Generated code; do not modify

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct HttpHeadersItem {
    pub name: String,
    #[serde(default)]
    pub value: Option<String>,
}
impl HttpHeadersItem {
    pub fn new(name: String) -> Self {
        Self { name, value: None }
    }
}
pub type HttpHeaders = Vec<HttpHeadersItem>;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "ignition-config")]
pub struct IgnitionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<Vec<Resource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<Resource>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "proxy")]
pub struct Proxy {
    #[serde(default)]
    #[serde(rename = "httpProxy")]
    pub http_proxy: Option<String>,
    #[serde(default)]
    #[serde(rename = "httpsProxy")]
    pub https_proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "noProxy")]
    pub no_proxy: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SecurityTls {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "certificateAuthorities")]
    pub certificate_authorities: Option<Vec<Resource>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "security")]
pub struct Security {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<SecurityTls>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "timeouts")]
pub struct Timeouts {
    #[serde(default)]
    #[serde(rename = "httpResponseHeaders")]
    pub http_response_headers: Option<i64>,
    #[serde(default)]
    #[serde(rename = "httpTotal")]
    pub http_total: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "ignition")]
pub struct Ignition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<IgnitionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<Proxy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Security>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeouts: Option<Timeouts>,
    pub version: String,
}
impl Ignition {
    pub fn new(version: String) -> Self {
        Self {
            config: None,
            proxy: None,
            security: None,
            timeouts: None,
            version,
        }
    }
}
pub type KernelArgument = String;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "kernelArguments")]
pub struct KernelArguments {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shouldExist")]
    pub should_exist: Option<Vec<KernelArgument>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shouldNotExist")]
    pub should_not_exist: Option<Vec<KernelArgument>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "group")]
pub struct Group {
    #[serde(default)]
    pub gid: Option<i64>,
    pub name: String,
    #[serde(default)]
    #[serde(rename = "passwordHash")]
    pub password_hash: Option<String>,
    #[serde(default)]
    #[serde(rename = "shouldExist")]
    pub should_exist: Option<bool>,
    #[serde(default)]
    pub system: Option<bool>,
}
impl Group {
    pub fn new(name: String) -> Self {
        Self {
            gid: None,
            name,
            password_hash: None,
            should_exist: None,
            system: None,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "user")]
pub struct User {
    #[serde(default)]
    pub gecos: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "homeDir")]
    pub home_dir: Option<String>,
    pub name: String,
    #[serde(default)]
    #[serde(rename = "noCreateHome")]
    pub no_create_home: Option<bool>,
    #[serde(default)]
    #[serde(rename = "noLogInit")]
    pub no_log_init: Option<bool>,
    #[serde(default)]
    #[serde(rename = "noUserGroup")]
    pub no_user_group: Option<bool>,
    #[serde(default)]
    #[serde(rename = "passwordHash")]
    pub password_hash: Option<String>,
    #[serde(default)]
    #[serde(rename = "primaryGroup")]
    pub primary_group: Option<String>,
    #[serde(default)]
    pub shell: Option<String>,
    #[serde(default)]
    #[serde(rename = "shouldExist")]
    pub should_exist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sshAuthorizedKeys")]
    pub ssh_authorized_keys: Option<Vec<String>>,
    #[serde(default)]
    pub system: Option<bool>,
    #[serde(default)]
    pub uid: Option<i64>,
}
impl User {
    pub fn new(name: String) -> Self {
        Self {
            gecos: None,
            groups: None,
            home_dir: None,
            name,
            no_create_home: None,
            no_log_init: None,
            no_user_group: None,
            password_hash: None,
            primary_group: None,
            shell: None,
            should_exist: None,
            ssh_authorized_keys: None,
            system: None,
            uid: None,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "passwd")]
pub struct Passwd {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "resource")]
pub struct Resource {
    #[serde(default)]
    pub compression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "httpHeaders")]
    pub http_headers: Option<HttpHeaders>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<Verification>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "cex")]
pub struct Cex {
    #[serde(default)]
    pub enabled: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "clevis")]
pub struct Clevis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<ClevisCustom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tang: Option<Vec<Tang>>,
    #[serde(default)]
    pub threshold: Option<i64>,
    #[serde(default)]
    #[serde(rename = "tpm2")]
    pub tpm2: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "clevisCustom")]
pub struct ClevisCustom {
    #[serde(default)]
    pub config: Option<String>,
    #[serde(default)]
    #[serde(rename = "needsNetwork")]
    pub needs_network: Option<bool>,
    #[serde(default)]
    pub pin: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "directory")]
pub struct Directory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<NodeGroup>,
    #[serde(default)]
    pub mode: Option<i64>,
    #[serde(default)]
    pub overwrite: Option<bool>,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<NodeUser>,
}
impl Directory {
    pub fn new(path: String) -> Self {
        Self {
            group: None,
            mode: None,
            overwrite: None,
            path,
            user: None,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "disk")]
pub struct Disk {
    pub device: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<Partition>>,
    #[serde(default)]
    #[serde(rename = "wipeTable")]
    pub wipe_table: Option<bool>,
}
impl Disk {
    pub fn new(device: String) -> Self {
        Self {
            device,
            partitions: None,
            wipe_table: None,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "file")]
pub struct File {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append: Option<Vec<Resource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Resource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<NodeGroup>,
    #[serde(default)]
    pub mode: Option<i64>,
    #[serde(default)]
    pub overwrite: Option<bool>,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<NodeUser>,
}
impl File {
    pub fn new(path: String) -> Self {
        Self {
            append: None,
            contents: None,
            group: None,
            mode: None,
            overwrite: None,
            path,
            user: None,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "filesystem")]
pub struct Filesystem {
    pub device: String,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mountOptions")]
    pub mount_options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default)]
    #[serde(rename = "wipeFilesystem")]
    pub wipe_filesystem: Option<bool>,
}
impl Filesystem {
    pub fn new(device: String) -> Self {
        Self {
            device,
            format: None,
            label: None,
            mount_options: None,
            options: None,
            path: None,
            uuid: None,
            wipe_filesystem: None,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "link")]
pub struct Link {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<NodeGroup>,
    #[serde(default)]
    pub hard: Option<bool>,
    #[serde(default)]
    pub overwrite: Option<bool>,
    pub path: String,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<NodeUser>,
}
impl Link {
    pub fn new(path: String) -> Self {
        Self {
            group: None,
            hard: None,
            overwrite: None,
            path,
            target: None,
            user: None,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "luks")]
pub struct Luks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cex: Option<Cex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clevis: Option<Clevis>,
    #[serde(default)]
    pub device: Option<String>,
    #[serde(default)]
    pub discard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyFile")]
    pub key_file: Option<Resource>,
    #[serde(default)]
    pub label: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "openOptions")]
    pub open_options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default)]
    #[serde(rename = "wipeVolume")]
    pub wipe_volume: Option<bool>,
}
impl Luks {
    pub fn new(name: String) -> Self {
        Self {
            cex: None,
            clevis: None,
            device: None,
            discard: None,
            key_file: None,
            label: None,
            name,
            open_options: None,
            options: None,
            uuid: None,
            wipe_volume: None,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct NodeGroup {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct NodeUser {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "node")]
pub struct Node {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<NodeGroup>,
    #[serde(default)]
    pub overwrite: Option<bool>,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<NodeUser>,
}
impl Node {
    pub fn new(path: String) -> Self {
        Self {
            group: None,
            overwrite: None,
            path,
            user: None,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "partition")]
pub struct Partition {
    #[serde(default)]
    pub guid: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(default)]
    pub resize: Option<bool>,
    #[serde(default)]
    #[serde(rename = "shouldExist")]
    pub should_exist: Option<bool>,
    #[serde(default)]
    #[serde(rename = "sizeMiB")]
    pub size_mib: Option<i64>,
    #[serde(default)]
    #[serde(rename = "startMiB")]
    pub start_mib: Option<i64>,
    #[serde(default)]
    #[serde(rename = "typeGuid")]
    pub type_guid: Option<String>,
    #[serde(default)]
    #[serde(rename = "wipePartitionEntry")]
    pub wipe_partition_entry: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "raid")]
pub struct Raid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
    #[serde(default)]
    pub level: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(default)]
    pub spares: Option<i64>,
}
impl Raid {
    pub fn new(name: String) -> Self {
        Self {
            devices: None,
            level: None,
            name,
            options: None,
            spares: None,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "tang")]
pub struct Tang {
    #[serde(default)]
    pub advertisement: Option<String>,
    #[serde(default)]
    pub thumbprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "storage")]
pub struct Storage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<Directory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<Disk>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<File>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystems: Option<Vec<Filesystem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub luks: Option<Vec<Luks>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid: Option<Vec<Raid>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "dropin")]
pub struct Dropin {
    #[serde(default)]
    pub contents: Option<String>,
    pub name: String,
}
impl Dropin {
    pub fn new(name: String) -> Self {
        Self {
            contents: None,
            name,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "unit")]
pub struct Unit {
    #[serde(default)]
    pub contents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropins: Option<Vec<Dropin>>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub mask: Option<bool>,
    pub name: String,
}
impl Unit {
    pub fn new(name: String) -> Self {
        Self {
            contents: None,
            dropins: None,
            enabled: None,
            mask: None,
            name,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "systemd")]
pub struct Systemd {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<Vec<Unit>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "verification")]
pub struct Verification {
    #[serde(default)]
    pub hash: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Config {
    pub ignition: Ignition,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "kernelArguments")]
    pub kernel_arguments: Option<KernelArguments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passwd: Option<Passwd>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<Storage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub systemd: Option<Systemd>,
}
impl Config {
    pub fn new(ignition: Ignition) -> Self {
        Self {
            ignition,
            kernel_arguments: None,
            passwd: None,
            storage: None,
            systemd: None,
        }
    }
}
