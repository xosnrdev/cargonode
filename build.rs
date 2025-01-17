use flate2::{write::GzEncoder, Compression};
use std::{
    env,
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
};
use tar::Builder;

const TEMPLATE_HASH: &str = "template.hash";
const EMBEDDING_MODULE: &str = "embedding.rs";
const PATH_PREFIX: &str = "assets/template/";

const ASSETS: &[&str] = &[
    "assets/template/package.json",
    "assets/template/src/main.js",
];

fn main() {
    set_git_revision_hash();
    set_windows_exe_options();

    for file in ASSETS {
        println!("cargo:rerun-if-changed={}", file);
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Could not get OUT_DIR"));
    let hash_file = out_dir.join(TEMPLATE_HASH);
    let embedding_module = out_dir.join(EMBEDDING_MODULE);

    compress_and_embed_templates(ASSETS, &hash_file, &embedding_module);

    println!("cargo:warning=TEMPLATE_OUT_DIR={}", out_dir.display());
}

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

/// Compresses given template files into a `.tar.gz` archive in memory,
/// and embeds it as a module. Tracks file changes with a hash file.
fn compress_and_embed_templates(assets: &[&str], hash_file: &Path, embedding_module: &Path) {
    for file in assets {
        println!("cargo:rerun-if-changed={}", file);
    }

    // Compute a hash of all file contents.
    let current_hash = compute_hash(assets);

    let previous_hash = if hash_file.exists() {
        let data = fs::read(hash_file)
            .unwrap_or_else(|_| panic!("Could not read hash file {}", hash_file.display()));
        if data.len() != 32 {
            panic!(
                "Hash file at {} has incorrect length: expected 32 bytes, found {} bytes",
                hash_file.display(),
                data.len()
            );
        }
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&data);
        hash
    } else {
        [0u8; 32]
    };

    // If the hash matches, skip the compression and embedding.
    if current_hash == previous_hash {
        println!("cargo:warning=No template changes detected. Skipping compression and embedding.");
        return;
    }
    println!("cargo:warning=Detected template changes. Recompressing and embedding.");

    let mut compressed_buffer = Vec::new();

    {
        // Create a GzEncoder that writes into `compressed_buffer`.
        let enc = GzEncoder::new(&mut compressed_buffer, Compression::fast());

        // Create a Tar builder using the GzEncoder as the writer.
        let mut tar_builder = Builder::new(enc);

        // Append each file to the tar archive.
        for file in assets {
            tar_builder
                .append_path_with_name(file, file.strip_prefix(PATH_PREFIX).unwrap_or_default())
                .unwrap_or_else(|_| panic!("Could not append file {} to archive", file));
        }

        tar_builder
            .finish()
            .expect("Could not finish tar.gz archive");
    }

    let mut embedded_file = File::create(embedding_module)
        .unwrap_or_else(|_| panic!("Could not create {}", embedding_module.display()));

    writeln!(
        embedded_file,
        "pub const EMBEDDED_TEMPLATE: &[u8] = &{:?};\n",
        compressed_buffer
    )
    .unwrap_or_else(|_| panic!("Could not write to {}", embedding_module.display()));

    let mut f = File::create(hash_file)
        .unwrap_or_else(|_| panic!("Could not create hash file {}", hash_file.display()));
    f.write_all(&current_hash)
        .unwrap_or_else(|_| panic!("Could not write to hash file {}", hash_file.display()));

    println!("cargo:rerun-if-changed={}", embedding_module.display());
}

fn compute_hash(files: &[&str]) -> [u8; 32] {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    const BUFFER_SIZE: usize = 8192;
    let mut buffer = [0u8; BUFFER_SIZE];

    for file in files {
        let f = File::open(file).unwrap_or_else(|_| panic!("Could not open file {}", file));
        let mut reader = BufReader::new(f);

        loop {
            let bytes_read = reader.read(&mut buffer).expect("Could not read file");
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
    }

    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}
