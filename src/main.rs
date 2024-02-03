use std::{env, future::pending};

use tokio::process;
use tracing::{error, info, Level};
use zbus::ConnectionBuilder;
use zbus_macros::dbus_interface;

struct FileManager {
    show_folder_program: String,
    show_items_program: String,
    show_properties_program: String,
}

#[dbus_interface(name = "org.freedesktop.FileManager1")]
impl FileManager {
    async fn show_folders(&self, ref uris: Vec<String>, _startup_id: &str) {
        process::Command::new("sh")
            .args([
                "-c",
                &self.show_folder_program.replace("%ARGS%", &uris.join(" ")),
            ])
            .spawn()
            .map_err(|e| {
                error!("Failed to run file manager: {}", e);
                e
            })
            .ok();
    }

    async fn show_items(&self, uris: Vec<String>, _startup_id: &str) {
        let cmd = self.show_items_program.replace("%ARGS%", &uris.join(" "));
        info!("Executing: {}", cmd);

        process::Command::new("sh")
            .args(["-c", &cmd])
            .spawn()
            .map_err(|e| {
                error!("Failed to run file manager: {}", e);
                e
            })
            .ok();
    }

    async fn show_item_properties(&self, ref uris: Vec<String>, _startup_id: &str) {
        process::Command::new("sh")
            .args([
                "-c",
                &self
                    .show_properties_program
                    .replace("%ARGS%", &uris.join(" ")),
            ])
            .spawn()
            .map_err(|e| {
                error!("Failed to run file manager: {}", e);
                e
            })
            .ok();
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let show_folder_program = env::var("DFMD_FOLDER_PROGRAM")
        .ok()
        .unwrap_or_else(|| r#"echo %ARGS% | xargs -n1 xdg-open"#.to_string());

    let show_items_program = env::var("DFMD_ITEMS_PROGRAM").ok().unwrap_or_else(|| 
        r#"echo %ARGS% | xargs -d " " -I {} sh -c 'p="{}"; echo "${p%/*}"' | xargs -n1 xdg-open"#.to_string(),
    );

    let show_properties_program = env::var("DFMD_PROPERTIES_PROGRAM")
        .ok()
        .unwrap_or_else(|| r#"echo %ARGS% | xargs -n1 xdg-open"#.to_string());

    let _conn = ConnectionBuilder::session()?
        .name("org.freedesktop.FileManager1")?
        .serve_at(
            "/org/freedesktop/FileManager1",
            FileManager {
                show_folder_program,
                show_items_program,
                show_properties_program,
            },
        )?
        .build()
        .await?;

    pending::<()>().await;

    Ok(())
}
