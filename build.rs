use std::{
    env, fs,
    path::Path,
    process::{Command, Output},
    time::SystemTime,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=web/");
    println!("cargo:rerun-if-changed=web/package.json");
    println!("cargo:rerun-if-changed=web/package-lock.json");
    println!("cargo:rerun-if-changed=web/app/");
    println!("cargo:rerun-if-changed=web/components/");
    println!("cargo:rerun-if-changed=web/lib/");
    println!("cargo:rerun-if-changed=web/hooks/");
    println!("cargo:rerun-if-changed=web/public/");
    println!("cargo:rerun-if-changed=web/next.config.js");
    println!("cargo:rerun-if-changed=web/tsconfig.json");
    println!("cargo:rerun-if-env-changed=SKIP_WEB_BUILD");

    // Check if we should skip web build
    if env::var("SKIP_WEB_BUILD").unwrap_or_default() == "1" {
        println!("cargo:warning=Skipping web build due to SKIP_WEB_BUILD=1");
        return Ok(());
    }

    let web_dir = Path::new("web");
    if !web_dir.exists() {
        println!("cargo:warning=Web directory not found, skipping frontend build");
        return Ok(());
    }

    // Check environment and build frontend
    match build_frontend() {
        Ok(_) => {
            println!("cargo:warning=Frontend build completed successfully");
        }
        Err(e) => {
            eprintln!("Frontend build failed: {}", e);
            // Don't fail the Rust build, just warn
            println!("cargo:warning=Frontend build failed: {}. Continuing with Rust build.", e);
        }
    }

    Ok(())
}

fn build_frontend() -> Result<(), BuildError> {
    let web_dir = Path::new("web");
    
    // Check build environment
    check_environment()?;
    
    // Check if package.json exists
    let package_json_path = web_dir.join("package.json");
    if !package_json_path.exists() {
        return Err(BuildError::MissingFile("web/package.json".to_string()));
    }
    
    // Check if we need to rebuild
    if !should_rebuild(web_dir)? {
        println!("cargo:warning=Frontend build cache is up to date, skipping build");
        return Ok(());
    }
    
    println!("cargo:warning=Building frontend...");
    
    // Run npm install if needed
    if should_npm_install(web_dir)? {
        println!("cargo:warning=Running npm install...");
        run_npm_command(web_dir, &["install"])?;
    }
    
    // Run npm build
    println!("cargo:warning=Running npm run build...");
    run_npm_command(web_dir, &["run", "build"])?;
    
    // Verify build output
    verify_build_output(web_dir)?;
    
    // Update build cache
    update_build_cache(web_dir)?;
    
    Ok(())
}

fn check_environment() -> Result<(), BuildError> {
    // Check Node.js
    let node_version = Command::new("node")
        .args(&["--version"])
        .output();
        
    match node_version {
        Ok(output) if output.status.success() => {
            let version_str = String::from_utf8_lossy(&output.stdout);
            let version_str = version_str.trim().trim_start_matches('v');
            
            if let Some(major_version) = version_str.split('.').next() {
                if let Ok(major) = major_version.parse::<u32>() {
                    if major < 18 {
                        return Err(BuildError::NodeVersionTooOld(major));
                    }
                } else {
                    return Err(BuildError::NodeVersionParseFailed(version_str.to_string()));
                }
            }
            
            println!("cargo:warning=Found Node.js version: {}", version_str);
        }
        Ok(_) => return Err(BuildError::NodeNotWorking),
        Err(_) => return Err(BuildError::NodeNotFound),
    }
    
    // Check pnpm first, fallback to npm
    let pnpm_version = Command::new("pnpm")
        .args(&["--version"])
        .output();
        
    match pnpm_version {
        Ok(output) if output.status.success() => {
            let version_str = String::from_utf8_lossy(&output.stdout);
            println!("cargo:warning=Found pnpm version: {}", version_str.trim());
            return Ok(());
        }
        _ => {}
    }
    
    // Fallback to npm
    let npm_version = Command::new("npm")
        .args(&["--version"])
        .output();
        
    match npm_version {
        Ok(output) if output.status.success() => {
            let version_str = String::from_utf8_lossy(&output.stdout);
            println!("cargo:warning=Found npm version: {}", version_str.trim());
        }
        Ok(_) => return Err(BuildError::NpmNotWorking),
        Err(_) => return Err(BuildError::PackageManagerNotFound),
    }
    
    Ok(())
}

fn should_rebuild(web_dir: &Path) -> Result<bool, BuildError> {
    let cache_file = web_dir.join(".build_cache");
    let dist_dir = web_dir.join("dist");
    
    // If dist doesn't exist, we need to build
    if !dist_dir.exists() {
        return Ok(true);
    }
    
    // If cache file doesn't exist, we need to build
    if !cache_file.exists() {
        return Ok(true);
    }
    
    // Get cache timestamp
    let cache_time = fs::metadata(&cache_file)
        .map_err(|e| BuildError::IoError(format!("Failed to read cache file metadata: {}", e)))?
        .modified()
        .map_err(|e| BuildError::IoError(format!("Failed to get cache file modified time: {}", e)))?;
    
    // Check if any source files are newer than cache
    let source_dirs = ["app", "components", "lib", "hooks", "public"];
    let source_files = ["package.json", "package-lock.json", "next.config.js", "tsconfig.json", "tailwind.config.ts"];
    
    for dir in &source_dirs {
        let dir_path = web_dir.join(dir);
        if dir_path.exists() {
            if is_dir_newer_than(&dir_path, cache_time)? {
                return Ok(true);
            }
        }
    }
    
    for file in &source_files {
        let file_path = web_dir.join(file);
        if file_path.exists() {
            let file_time = fs::metadata(&file_path)
                .map_err(|e| BuildError::IoError(format!("Failed to read {} metadata: {}", file, e)))?
                .modified()
                .map_err(|e| BuildError::IoError(format!("Failed to get {} modified time: {}", file, e)))?;
                
            if file_time > cache_time {
                return Ok(true);
            }
        }
    }
    
    Ok(false)
}

fn is_dir_newer_than(dir: &Path, cache_time: SystemTime) -> Result<bool, BuildError> {
    if !dir.is_dir() {
        return Ok(false);
    }
    
    let entries = fs::read_dir(dir)
        .map_err(|e| BuildError::IoError(format!("Failed to read directory {}: {}", dir.display(), e)))?;
        
    for entry in entries {
        let entry = entry
            .map_err(|e| BuildError::IoError(format!("Failed to read directory entry: {}", e)))?;
        let path = entry.path();
        
        if path.is_file() {
            let file_time = entry
                .metadata()
                .map_err(|e| BuildError::IoError(format!("Failed to read file metadata: {}", e)))?
                .modified()
                .map_err(|e| BuildError::IoError(format!("Failed to get file modified time: {}", e)))?;
                
            if file_time > cache_time {
                return Ok(true);
            }
        } else if path.is_dir() {
            // Recursively check subdirectories
            if is_dir_newer_than(&path, cache_time)? {
                return Ok(true);
            }
        }
    }
    
    Ok(false)
}

fn should_npm_install(web_dir: &Path) -> Result<bool, BuildError> {
    let package_lock = web_dir.join("package-lock.json");
    let pnpm_lock = web_dir.join("pnpm-lock.yaml");
    let node_modules = web_dir.join("node_modules");
    
    // If node_modules doesn't exist, we need to install
    if !node_modules.exists() {
        return Ok(true);
    }
    
    // Check pnpm-lock.yaml first
    if pnpm_lock.exists() {
        let lock_time = fs::metadata(&pnpm_lock)
            .map_err(|e| BuildError::IoError(format!("Failed to read pnpm-lock.yaml metadata: {}", e)))?
            .modified()
            .map_err(|e| BuildError::IoError(format!("Failed to get pnpm-lock.yaml modified time: {}", e)))?;
            
        let modules_time = fs::metadata(&node_modules)
            .map_err(|e| BuildError::IoError(format!("Failed to read node_modules metadata: {}", e)))?
            .modified()
            .map_err(|e| BuildError::IoError(format!("Failed to get node_modules modified time: {}", e)))?;
            
        if lock_time > modules_time {
            return Ok(true);
        }
    }
    // Fallback to package-lock.json if it exists
    else if package_lock.exists() {
        let lock_time = fs::metadata(&package_lock)
            .map_err(|e| BuildError::IoError(format!("Failed to read package-lock.json metadata: {}", e)))?
            .modified()
            .map_err(|e| BuildError::IoError(format!("Failed to get package-lock.json modified time: {}", e)))?;
            
        let modules_time = fs::metadata(&node_modules)
            .map_err(|e| BuildError::IoError(format!("Failed to read node_modules metadata: {}", e)))?
            .modified()
            .map_err(|e| BuildError::IoError(format!("Failed to get node_modules modified time: {}", e)))?;
            
        if lock_time > modules_time {
            return Ok(true);
        }
    }
    
    Ok(false)
}

fn run_npm_command(web_dir: &Path, args: &[&str]) -> Result<Output, BuildError> {
    // Try pnpm first
    if let Ok(output) = Command::new("pnpm")
        .args(args)
        .current_dir(web_dir)
        .output()
    {
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(BuildError::NpmCommandFailed {
                command: format!("pnpm {}", args.join(" ")),
                stdout: stdout.to_string(),
                stderr: stderr.to_string(),
                exit_code: output.status.code(),
            });
        }
        return Ok(output);
    }
    
    // Fallback to npm
    let output = Command::new("npm")
        .args(args)
        .current_dir(web_dir)
        .output()
        .map_err(|e| BuildError::CommandFailed(format!("Failed to execute npm: {}", e)))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(BuildError::NpmCommandFailed {
            command: format!("npm {}", args.join(" ")),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            exit_code: output.status.code(),
        });
    }
    
    Ok(output)
}

fn verify_build_output(web_dir: &Path) -> Result<(), BuildError> {
    let dist_dir = web_dir.join("dist");
    
    if !dist_dir.exists() {
        return Err(BuildError::BuildOutputMissing("dist directory not found".to_string()));
    }
    
    // Check for index.html
    let index_html = dist_dir.join("index.html");
    if !index_html.exists() {
        return Err(BuildError::BuildOutputMissing("dist/index.html not found".to_string()));
    }
    
    // Check for assets directory (common in Vite builds)
    let assets_dir = dist_dir.join("assets");
    if assets_dir.exists() {
        let assets_entries = fs::read_dir(&assets_dir)
            .map_err(|e| BuildError::IoError(format!("Failed to read assets directory: {}", e)))?;
        
        let has_files = assets_entries.count() > 0;
        if !has_files {
            println!("cargo:warning=Assets directory exists but is empty");
        }
    }
    
    println!("cargo:warning=Build output verification passed");
    Ok(())
}

fn update_build_cache(web_dir: &Path) -> Result<(), BuildError> {
    let cache_file = web_dir.join(".build_cache");
    fs::write(&cache_file, "")
        .map_err(|e| BuildError::IoError(format!("Failed to update build cache: {}", e)))?;
    Ok(())
}

#[derive(Debug)]
enum BuildError {
    NodeNotFound,
    NodeNotWorking,
    NodeVersionTooOld(u32),
    NodeVersionParseFailed(String),
    NpmNotFound,
    NpmNotWorking,
    PackageManagerNotFound,
    MissingFile(String),
    IoError(String),
    CommandFailed(String),
    NpmCommandFailed {
        command: String,
        stdout: String,
        stderr: String,
        exit_code: Option<i32>,
    },
    BuildOutputMissing(String),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::NodeNotFound => {
                write!(f, "Node.js not found. Please install Node.js version 18 or higher.\n\
                          Download from: https://nodejs.org/")
            }
            BuildError::NodeNotWorking => {
                write!(f, "Node.js is installed but not working properly. Please check your installation.")
            }
            BuildError::NodeVersionTooOld(version) => {
                write!(f, "Node.js version {} is too old. Please install Node.js version 18 or higher.\n\
                          Download from: https://nodejs.org/", version)
            }
            BuildError::NodeVersionParseFailed(version) => {
                write!(f, "Failed to parse Node.js version: {}. Please check your Node.js installation.", version)
            }
            BuildError::NpmNotFound => {
                write!(f, "npm not found. Please install npm or ensure it's in your PATH.")
            }
            BuildError::NpmNotWorking => {
                write!(f, "npm is installed but not working properly. Please check your npm installation.")
            }
            BuildError::PackageManagerNotFound => {
                write!(f, "No package manager found. Please install npm or pnpm.")
            }
            BuildError::MissingFile(file) => {
                write!(f, "Required file not found: {}. Please ensure the frontend project is properly set up.", file)
            }
            BuildError::IoError(msg) => {
                write!(f, "I/O error: {}", msg)
            }
            BuildError::CommandFailed(msg) => {
                write!(f, "Command execution failed: {}", msg)
            }
            BuildError::NpmCommandFailed { command, stdout, stderr, exit_code } => {
                write!(f, "npm command failed: {}\n\
                          Exit code: {:?}\n\
                          Stdout: {}\n\
                          Stderr: {}", command, exit_code, stdout, stderr)
            }
            BuildError::BuildOutputMissing(msg) => {
                write!(f, "Build output verification failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for BuildError {}