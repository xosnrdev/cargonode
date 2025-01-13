use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use flate2::{write::GzEncoder, Compression};
use tar::Builder;

//----------------------------------------------------------------------
// Constants
//----------------------------------------------------------------------

const TEMPLATE_FILES: &[&str] = &[
    "assets/template/package.json",
    "assets/template/src/main.js",
];
const TEMPLATE_HASH_PATH: &str = "template.hash";
const EMBEDDED_MODULE_PATH: &str = "embedded_template.rs";
const DESTINATION_PATH: &str = "template.tar.gz";

//----------------------------------------------------------------------
// Main Function
//----------------------------------------------------------------------

fn main() -> Result<()> {
    set_git_revision_hash();
    set_windows_exe_options();

    for file in TEMPLATE_FILES {
        println!("cargo:rerun-if-changed={}", file);
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let dest_path = out_dir.join(DESTINATION_PATH);
    let hash_path = out_dir.join(TEMPLATE_HASH_PATH);
    let embedded_module_path = out_dir.join(EMBEDDED_MODULE_PATH);

    compress_and_embed_templates(
        TEMPLATE_FILES,
        &dest_path,
        &hash_path,
        &embedded_module_path,
    )?;

    // HACK: For debugging purpose print the `out_dir`
    println!("cargo:warning=TEMPLATE_OUT_DIR={}", out_dir.display());

    Ok(())
}

//----------------------------------------------------------------------
// Functions
//----------------------------------------------------------------------

/// Embed a Windows manifest and set some linker options.
///
/// The main reason for this is to enable long path support on Windows. This
/// still, I believe, requires enabling long path support in the registry. But
/// if that's enabled, then this will let cargonode use C:\... style paths that
/// are longer than 260 characters.
fn set_windows_exe_options() {
    static MANIFEST: &str = "pkg/windows/Manifest.xml";

    let Ok(target_os) = std::env::var("CARGO_CFG_TARGET_OS") else {
        return;
    };
    let Ok(target_env) = std::env::var("CARGO_CFG_TARGET_ENV") else {
        return;
    };
    if !(target_os == "windows" && target_env == "msvc") {
        return;
    }

    let Ok(mut manifest) = std::env::current_dir() else {
        return;
    };
    manifest.push(MANIFEST);
    let Some(manifest) = manifest.to_str() else {
        return;
    };

    println!("cargo:rerun-if-changed={}", MANIFEST);
    // Embed the Windows application manifest file.
    println!("cargo:rustc-link-arg-bin=cargonode=/MANIFEST:EMBED");
    println!("cargo:rustc-link-arg-bin=cargonode=/MANIFESTINPUT:{manifest}");
    // Turn linker warnings into errors. Helps debugging, otherwise the
    // warnings get squashed (I believe).
    println!("cargo:rustc-link-arg-bin=cargonode=/WX");
}

/// Make the current git hash available to the build as the environment
/// variable `CARGONODE_BUILD_GIT_HASH`.
fn set_git_revision_hash() {
    use std::process::Command;

    let args = &["rev-parse", "--short=10", "HEAD"];
    let Ok(output) = Command::new("git").args(args).output() else {
        return;
    };
    let rev = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if rev.is_empty() {
        return;
    }
    println!("cargo:rustc-env=CARGONODE_BUILD_GIT_HASH={}", rev);
}

/// Compresses given template files into a `.tar.gz` archive
/// and embeds it as a module.
fn compress_and_embed_templates(
    files: &[&str],
    destination: &Path,
    hash_file: &Path,
    embedded_module: &Path,
) -> Result<()> {
    let current_hash = compute_hash(files)?;

    // Read the previous hash if it exists
    let previous_hash = if hash_file.exists() {
        fs::read(hash_file).context("Failed to read previous hash file")?
    } else {
        Vec::new()
    };

    // Compare hashes to decide whether to compress and embed
    if current_hash != previous_hash {
        println!("cargo:warning=Template changed. Compressing and embedding.");

        // Create the compressed archive
        let tar_gz = File::create(destination).context("Could not create tar.gz file")?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = Builder::new(enc);

        for &file in files {
            tar.append_path(file)
                .with_context(|| format!("Could not append {}", file))?;
        }

        tar.finish().context("Could not finish tar.gz")?;

        // Read the compressed data
        let mut compressed_data = Vec::new();
        File::open(destination)
            .context("Could not open compressed template file")?
            .read_to_end(&mut compressed_data)
            .context("Could not read compressed template file")?;

        let mut embedded_file = File::create(embedded_module)
            .with_context(|| format!("Could not create {} file", EMBEDDED_MODULE_PATH))?;

        writeln!(
            embedded_file,
            "pub const EMBEDDED_TEMPLATE: &[u8] = {:?};\n",
            compressed_data
        )
        .context("Could not write to embedded_template.rs")?;

        // Update the stored hash
        let mut f = File::create(hash_file).context("Could not create hash file")?;
        f.write_all(&current_hash)
            .context("Could not write hash to file")?;
    } else {
        println!("cargo:warning=Template unchanged. Skipping compression and embedding.");
    }

    println!("cargo:rerun-if-changed={}", embedded_module.display());

    Ok(())
}

/// Computes a SHA-256 hash of the given files.
fn compute_hash(files: &[&str]) -> Result<Vec<u8>> {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();

    for &file in files {
        let mut f = File::open(file).with_context(|| format!("Could not open file {}", file))?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)
            .with_context(|| format!("Could not read file {}", file))?;
        hasher.update(&buffer);
    }

    Ok(hasher.finalize().to_vec())
}
