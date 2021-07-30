// Generated code; do not modify

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct HttpHeadersItem {
    pub name: String,
    #[serde(default)]
    pub value: Option<String>,
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
    pub no_proxy: Option<Vec<Option<String>>>,
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
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
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
    pub system: Option<bool>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sshAuthorizedKeys")]
    pub ssh_authorized_keys: Option<Vec<String>>,
    #[serde(default)]
    pub system: Option<bool>,
    #[serde(default)]
    pub uid: Option<i64>,
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
pub struct DirectoryGroup {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DirectoryGroupUser {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "directory")]
pub struct Directory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<DirectoryGroup>,
    #[serde(default)]
    pub mode: Option<i64>,
    #[serde(default)]
    pub overwrite: Option<bool>,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<DirectoryGroupUser>,
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
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct FileItemGroup {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct FileItemGroupUser {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "file")]
pub struct File {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append: Option<Vec<Resource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Resource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<FileItemGroup>,
    #[serde(default)]
    pub mode: Option<i64>,
    #[serde(default)]
    pub overwrite: Option<bool>,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<FileItemGroupUser>,
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
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct LinkGroup {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct LinkGroupUser {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "link")]
pub struct Link {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<LinkGroup>,
    #[serde(default)]
    pub hard: Option<bool>,
    #[serde(default)]
    pub overwrite: Option<bool>,
    pub path: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<LinkGroupUser>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct NodeGroup {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct NodeGroupUser {
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
    pub user: Option<NodeGroupUser>,
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
    pub devices: Vec<String>,
    pub level: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(default)]
    pub spares: Option<i64>,
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
    pub raid: Option<Vec<Raid>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "dropin")]
pub struct Dropin {
    #[serde(default)]
    pub contents: Option<String>,
    pub name: String,
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
    pub passwd: Option<Passwd>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<Storage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub systemd: Option<Systemd>,
}
