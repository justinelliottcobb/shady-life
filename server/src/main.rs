use actix_files as fs;
use actix_web::{App, HttpServer, middleware};
use std::path::PathBuf;
use std::process::Command;

fn list_directory_recursive(path: &PathBuf, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}Contents of {}:", indent, path.display());
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    list_directory_recursive(&path, depth + 1);
                } else {
                    println!("{}  {}", indent, path.file_name().unwrap().to_string_lossy());
                }
            }
        }
    } else {
        println!("{}  (Unable to read directory)", indent);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let workspace_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Failed to find workspace directory")
        .to_path_buf();
    
    let client_dir = workspace_dir.join("client");
    let client_dist = client_dir.join("dist");
    
    println!("\nPreparing build...");
    println!("Client directory: {}", client_dir.display());
    println!("Dist directory: {}", client_dist.display());

    // Clean up the dist directory if it exists
    if client_dist.exists() {
        println!("Cleaning existing dist directory...");
        std::fs::remove_dir_all(&client_dist).expect("Failed to clean dist directory");
    }
    std::fs::create_dir_all(&client_dist).expect("Failed to create dist directory");
    
    // Build the client using wasm-pack with full output capture
    println!("\nBuilding WebAssembly client...");
    let output = Command::new("wasm-pack")
        .current_dir(&client_dir)
        .args(&["build", "--target", "web", "--out-dir", "dist", "--verbose"])
        .output()
        .expect("Failed to execute wasm-pack");

    // Print the output regardless of success/failure
    println!("\nwasm-pack stdout:");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("\nwasm-pack stderr:");
    println!("{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        panic!("Failed to build WebAssembly client");
    }

    println!("\nCopying index.html to dist directory...");
    std::fs::copy(
        client_dir.join("index.html"),
        client_dist.join("index.html"),
    ).expect("Failed to copy index.html");

    println!("\nFinal directory structure in {}:", client_dist.display());
    list_directory_recursive(&client_dist, 0);
    
    // Also check pkg directory in case files ended up there
    let pkg_dir = client_dir.join("pkg");
    if pkg_dir.exists() {
        println!("\nContents of pkg directory found at {}:", pkg_dir.display());
        list_directory_recursive(&pkg_dir, 0);
    }
    
    println!("\nStarting server...");
    log::info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/", client_dist.clone())
                .index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
