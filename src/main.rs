#![windows_subsystem = "windows"]
use druid::widget::{Button, Flex, Label, TextBox, Align};
use druid::{AppLauncher, Data, Env, Lens, Widget, WidgetExt, WindowDesc};
use native_dialog::FileDialog;
use std::{fs, path::Path};
use walkdir::WalkDir;

#[derive(Data, Clone, Lens)]
struct AppState {
    target_folder: Option<String>,
    source_folder: Option<String>,
    extension_input: String,
}


fn main() {
    let window = WindowDesc::new(build_ui()) 
        .title("HDD Checker")
        .window_size((600.0,400.0));

    let initial_state = AppState {
        target_folder: None,
        source_folder: None,
        extension_input: String::new(),
    };

    AppLauncher::with_window(window)
        .launch(initial_state)
        .expect("The application failed to open.");
}


fn build_ui() -> impl Widget<AppState> {
    let source_label = Label::new(|data: &AppState, _env: &Env| {
        if let Some(ref folder) = data.source_folder {
            format!("Selected folder: {}", folder)
        } else {
            "No folder selected".to_string()
        }
    });

    let select_source_folder = Button::new("Select source folder")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| {
            if let Some(path) = FileDialog::new().show_open_single_dir().ok().flatten() {
                data.source_folder = Some(path.display().to_string());
            }
        });

    let target_label = Label::new(|data: &AppState, _env: &Env| {
        if let Some(ref folder) = data.target_folder {
            format!("Selected folder: {}", folder)
        } else {
            "No folder selected".to_string()
        }
    });

    let select_target_folder = Button::new("Select destination folder")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| {
            if let Some(path) = FileDialog::new().show_open_single_dir().ok().flatten() {
                data.target_folder = Some(path.display().to_string());
            }
        });

    let extension_label = Label::new("File extensions");

    let extension_box = TextBox::new()
        .with_placeholder("e.g. jpg,png,gif")
        .lens(AppState::extension_input);

    let copy_button = Button::new("Copy files")
        .on_click(|_ctx, data: &mut AppState, _env: &Env| {
            if let (Some(source_folder), Some(target_folder)) = (&data.source_folder, &data.target_folder) {
                let source_path = Path::new(source_folder);
                let target_folder = Path::new(target_folder);

                if let Err(e) = copy_files_with_extension(source_path, target_folder, &data.extension_input) {
                    eprintln!("Error copying files: {}", e);
                }
            }
        });
    

    let layout = Flex::column()
        .with_child(source_label)
        .with_child(select_source_folder)
        .with_spacer(20.0)
        .with_child(target_label)
        .with_child(select_target_folder)
        .with_spacer(20.0)
        .with_child(extension_label)
        .with_child(extension_box)
        .with_spacer(50.0)
        .with_child(copy_button);


    Align::centered(layout)
}

fn copy_files_with_extension(source: &Path, destination: &Path, extensions: &str) -> std::io::Result<()> {
    let parts: Vec<&str> = extensions.split(",").collect();
    println!("{:?}", parts);

    if source.is_dir() {
        for entry in WalkDir::new(source) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {  
                println!("I found a file in: {}", path.display());
                if let Some(extension) = path.extension() {
                    if parts.contains(&extension.to_str().unwrap()) {
                        let file_name = path.file_name().unwrap();
                        let dest_path = Path::new(destination).join(file_name);
                        println!("Correct filetype found in: {}, copying file to destination folder...", dest_path.display());
                        fs::copy(&path, &dest_path).unwrap();
                    }
                }
            }
        } 
        println!("Finished file copy");
    }
    Ok(())
}
