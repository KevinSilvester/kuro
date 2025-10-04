use indexmap::IndexMap;

use super::v1;

#[derive(Debug, Clone)]
pub enum DotFileLocalMethod {
    Copy,
    Symlink,
}

#[derive(Debug, Clone)]
pub struct DotfileGit {
    pub name: String,
    pub repo: String,
    pub branch: Option<String>,
    pub commit: Option<String>,
    pub dest: String,
}

#[derive(Debug, Clone)]
pub struct DotfileLocal {
    pub name: String,
    pub src: String,
    pub dest: String,
    pub method: DotFileLocalMethod,
}

#[derive(Debug, Clone)]
pub struct DotfileHttp {
    pub name: String,
    pub url: String,
    pub dest: String,
}

#[derive(Debug, Clone)]
pub struct Dotfiles {
    pub git: IndexMap<String, DotfileGit>,
    pub local: IndexMap<String, DotfileLocal>,
    pub http: IndexMap<String, DotfileHttp>,
}

#[derive(Debug, Clone)]
pub struct ConfigInternal {
    pub env: IndexMap<String, String>,
    pub editor: Option<String>,
    pub trash_cmd: Option<Vec<String>>,
    pub dotfiles: Dotfiles,
}
