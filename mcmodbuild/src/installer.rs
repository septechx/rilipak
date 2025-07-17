use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use crate::structs::{BuildType, ExcludeType, ModBuild};
use anyhow::{bail, Result};
use directories::ProjectDirs;

pub struct Installer {
    pub build: ModBuild,
    pub cache_dir: PathBuf,
    pub build_path: PathBuf,
}

impl Installer {
    pub fn new(build: ModBuild) -> Result<Self> {
        let dirs = ProjectDirs::from("com", "siesque", "mcmodbuild")
            .ok_or_else(|| anyhow::anyhow!("Could not determine cache directory"))?;

        let cache_dir = dirs.cache_dir().to_path_buf();
        let build_path = cache_dir.join(format!("{}-{}", build.id, build.branch));

        Ok(Self {
            build,
            cache_dir,
            build_path,
        })
    }

    #[allow(dead_code)]
    pub fn install(&self, destination: PathBuf) -> Result<()> {
        println!("🚀 Starting installation...");

        let steps: Vec<(&str, &str, &str, Box<dyn FnOnce() -> Result<()>>)> = vec![
            (
                "Preparing cache...\n",
                "Created build directory\n",
                "Failed to create build directory\n",
                Box::new(|| self.ensure_cache_directory()),
            ),
            (
                "Updating repository...\n",
                "Repository synced\n",
                "Failed to sync recpository\n",
                Box::new(|| self.clone_or_update_repository()),
            ),
            (
                "Building mod...\n",
                "Mod built\n",
                "Failed to build mod\n",
                Box::new(|| self.build_project()),
            ),
            (
                "Copying files...\n",
                "Files copied\n",
                "Failed to copy files\n",
                Box::new(move || self.copy_built_files(&destination)),
            ),
        ];

        for (i, (msg, success, failure, step)) in steps.into_iter().enumerate() {
            print!("[{}/{}] {}", i + 1, 4, msg);
            std::io::stdout().flush()?;

            match step() {
                Ok(_) => println!("✅ {success}"),
                Err(e) => {
                    println!("❌ {failure}");
                    return Err(e);
                }
            }
        }

        println!("🎉 Installation complete!");
        Ok(())
    }

    pub fn ensure_cache_directory(&self) -> Result<()> {
        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }

    pub fn clone_or_update_repository(&self) -> Result<()> {
        let exists = self.build_path.exists();

        if !exists {
            fs::create_dir(&self.build_path)?;
        }

        let mut git_command = if exists {
            self.create_git_pull_command()
        } else {
            self.create_git_clone_command()
        };

        git_command.spawn()?.wait()?;
        Ok(())
    }

    pub fn build_project(&self) -> Result<()> {
        let mut build_command = self.create_build_command()?;
        build_command.spawn()?.wait()?;
        Ok(())
    }

    fn create_git_clone_command(&self) -> Command {
        let mut cmd = Command::new("git");
        cmd.arg("clone")
            .arg(&self.build.git)
            .arg(&self.build_path)
            .arg("-b")
            .arg(&self.build.branch)
            .arg("--depth")
            .arg("1")
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());
        cmd
    }

    fn create_git_pull_command(&self) -> Command {
        let mut cmd = Command::new("git");
        cmd.arg("pull")
            .current_dir(&self.build_path)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());
        cmd
    }

    fn create_build_command(&self) -> Result<Command> {
        let cmd_str = match self.build.build {
            BuildType::Std => "./gradlew build".to_string(),
            BuildType::Cmd => self
                .build
                .cmd
                .clone()
                .ok_or_else(|| anyhow::anyhow!("Custom command not specified"))?,
        };

        let parts: Vec<&str> = cmd_str.split_whitespace().collect();
        let (program, args) = parts
            .split_first()
            .ok_or_else(|| anyhow::anyhow!("Invalid command given"))?;

        let mut cmd = Command::new(program);
        cmd.args(args)
            .current_dir(&self.build_path)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());

        Ok(cmd)
    }

    pub fn get_built_files(&self) -> Result<PathBuf> {
        let real_out = self
            .build
            .out
            .replace('@', self.build_path.to_str().unwrap());

        let path_type;

        let out_path = if real_out.starts_with("file:") {
            path_type = PathType::File;
            real_out.strip_prefix("file:").unwrap()
        } else {
            path_type = PathType::Dir;
            real_out.strip_prefix("dir:").unwrap()
        };

        let out_path = Path::new(&out_path);

        let path: PathBuf = match path_type {
            PathType::File => out_path.to_path_buf(),
            PathType::Dir => {
                let matching_files = self.find_matching_files(out_path)?;

                if matching_files.len() != 1 {
                    bail!("Matched more or less than one result files");
                }

                matching_files[0].clone()
            }
        };

        Ok(path)
    }

    #[allow(dead_code)]
    fn copy_built_files(&self, destination: &PathBuf) -> Result<()> {
        let path = self.get_built_files()?;
        fs::copy(path, destination)?;

        Ok(())
    }

    fn find_matching_files(&self, out_path: &Path) -> Result<Vec<PathBuf>> {
        let mut matching_files = Vec::new();

        for entry in fs::read_dir(out_path)? {
            let file = entry?;
            if !self.should_exclude_file(&file.file_name().to_string_lossy()) {
                matching_files.push(file.path());
            }
        }

        Ok(matching_files)
    }

    fn should_exclude_file(&self, filename: &str) -> bool {
        for filter in &self.build.exclude {
            match filter.type_name {
                ExcludeType::Starts => {
                    if filename.starts_with(&filter.value) {
                        return true;
                    }
                }
                ExcludeType::Ends => {
                    if filename.ends_with(&filter.value) {
                        return true;
                    }
                }
                ExcludeType::Contains => {
                    if filename.contains(&filter.value) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

enum PathType {
    File,
    Dir,
}
