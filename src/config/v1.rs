use indexmap::IndexMap;
use serde::Deserialize;

use super::internal;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Value<T> {
    Simple(T),
    Specific {
        windows: Option<T>,
        linux: Option<T>,
        macos: Option<T>,
        unix: Option<T>,
    },
}

impl<T: Clone> Value<T> {
    pub fn resovle(&self) -> Option<T> {
        match self {
            Value::Simple(v) => Some(v.clone()),
            Value::Specific {
                windows,
                linux,
                macos,
                unix,
            } => self.resolve_specific(windows, linux, macos, unix),
        }
    }

    fn resolve_specific(
        &self,
        windows: &Option<T>,
        linux: &Option<T>,
        macos: &Option<T>,
        unix: &Option<T>,
    ) -> Option<T> {
        match std::env::consts::OS {
            "windows" => windows.clone(),
            "linux" => linux.clone().or(unix.clone()),
            "macos" => macos.clone().or(unix.clone()),
            "unix" => unix.clone(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum DotFileLocalMethod {
    Copy,
    Symlink,
}

#[derive(Debug, Clone, Deserialize)]
struct DotfileGit {
    pub name: String,
    pub repo: Value<String>,
    pub branch: Option<Value<String>>,
    pub commit: Option<Value<String>>,
    pub dest: Value<String>,
}

impl DotfileGit {
    pub fn to_internal(&self) -> anyhow::Result<internal::DotfileGit> {
        let repo = self.repo.resovle().ok_or(anyhow::anyhow!(
            "Failed to resolve repo URL for git dotfile: {}",
            self.name
        ))?;
        let branch = self.branch.as_ref().and_then(|b| b.resovle());
        let commit = self.commit.as_ref().and_then(|c| c.resovle());
        let dest = self.dest.resovle().ok_or(anyhow::anyhow!(
            "Failed to resolve dest path for git dotfile: {}",
            self.name
        ))?;
        Ok(internal::DotfileGit {
            name: self.name.clone(),
            repo,
            branch,
            commit,
            dest,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
struct DotfileLocal {
    pub name: String,
    pub src: Value<String>,
    pub dest: Value<String>,
    pub method: Value<DotFileLocalMethod>,
}

impl DotfileLocal {
    pub fn to_internal(&self) -> anyhow::Result<internal::DotfileLocal> {
        let src = self.src.resovle().ok_or(anyhow::anyhow!(
            "Failed to resolve src path for local dotfile: {}",
            self.name
        ))?;
        let dest = self.dest.resovle().ok_or(anyhow::anyhow!(
            "Failed to resolve dest path for local dotfile: {}",
            self.name
        ))?;
        let method = self.method.resovle().ok_or(anyhow::anyhow!(
            "Failed to resolve method for local dotfile: {}",
            self.name
        ))?;
        let method = match method {
            DotFileLocalMethod::Copy => internal::DotFileLocalMethod::Copy,
            DotFileLocalMethod::Symlink => internal::DotFileLocalMethod::Symlink,
        };
        Ok(internal::DotfileLocal {
            name: self.name.clone(),
            src,
            dest,
            method,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
struct DotfileHttp {
    pub name: String,
    pub url: Value<String>,
    pub dest: Value<String>,
}

impl DotfileHttp {
    pub fn to_internal(&self) -> anyhow::Result<internal::DotfileHttp> {
        let url = self.url.resovle().ok_or(anyhow::anyhow!(
            "Failed to resolve URL for HTTP dotfile: {}",
            self.name
        ))?;
        let dest = self.dest.resovle().ok_or(anyhow::anyhow!(
            "Failed to resolve dest path for HTTP dotfile: {}",
            self.name
        ))?;
        Ok(internal::DotfileHttp {
            name: self.name.clone(),
            url,
            dest,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Dotfiles {
    pub git: Vec<DotfileGit>,
    pub local: Vec<DotfileLocal>,
    pub http: Vec<DotfileHttp>,
}

impl Dotfiles {
    pub fn to_internal(&self) -> anyhow::Result<internal::Dotfiles> {
        let mut git_map = IndexMap::new();
        for git in self.git.iter() {
            let internal_git = git.to_internal()?;
            git_map.insert(internal_git.name.clone(), internal_git);
        }
        let mut local_map = IndexMap::new();
        for local in self.local.iter() {
            let internal_local = local.to_internal()?;
            local_map.insert(internal_local.name.clone(), internal_local);
        }
        let mut http_map = IndexMap::new();
        for http in self.http.iter() {
            let internal_http = http.to_internal()?;
            http_map.insert(internal_http.name.clone(), internal_http);
        }
        Ok(internal::Dotfiles {
            git: git_map,
            local: local_map,
            http: http_map,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Options {
    pub editor: Option<Value<String>>,
    pub trash_cmd: Option<Value<Vec<String>>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigV1 {
    env: IndexMap<String, Value<String>>,
    #[serde(rename = "kuro_options")]
    options: Options,
    dotfiles: Dotfiles,
}

impl ConfigV1 {
    pub fn to_internal(&self) -> anyhow::Result<internal::ConfigInternal> {
        let mut env = IndexMap::new();
        for (k, v) in self.env.iter() {
            if let Some(resolved) = v.resovle() {
                env.insert(k.clone(), resolved);
            }
        }

        let editor = self.options.editor.as_ref().and_then(|e| e.resovle());

        let trash_cmd = self.options.trash_cmd.as_ref().and_then(|t| t.resovle());

        let dotfiles = self.dotfiles.to_internal()?;

        Ok(internal::ConfigInternal {
            env,
            editor,
            trash_cmd,
            dotfiles,
        })
    }
}
