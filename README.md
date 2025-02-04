# HDD-Checker

`HDD-Checker` is a Rust-based application that scans a selected source folder for files with specific extensions and copies them to a designated target folder. The application uses a simple graphical user interface (GUI) built with the `druid` library and performs file operations using the `walkdir` and `std::fs` libraries.

## Features

- Select source and target folders using a file dialog.
- Input file extensions to filter which files will be copied (e.g., `jpg,png,gif`).
- Scan through the source folder (and subfolders) and copy matching files to the target folder.

## Dependencies

- **druid**: A Rust-native GUI library to build cross-platform graphical applications.
- **native_dialog**: A library for simple file dialogs.
- **walkdir**: A library to recursively walk through directories.
- **std::fs**: Standard library for filesystem operations.

## How It Works

### 1. **User Interface**

The user interface is built using `druid`. It contains the following elements:

- **Source Folder Selection**: A button labeled "Velg hovedmappe" that opens a file dialog for the user to select the source folder where files will be scanned.
- **Target Folder Selection**: A button labeled "Velg destinasjonsmappe" that opens a file dialog for the user to select the destination folder where matching files will be copied.
- **File Type Input**: A text box for the user to input file extensions (e.g., `jpg,png,gif`). The program will only copy files with the specified extensions.
- **Copy Files Button**: A button labeled "Kopier filer" that, when clicked, triggers the process of scanning the source folder and copying matching files to the target folder.

### 2. **Application State**

The state of the application is managed using the `AppState` struct, which stores:
- `source_folder`: The path to the source folder.
- `target_folder`: The path to the destination folder.
- `extension_input`: A comma-separated list of file extensions that the user wants to copy (e.g., `jpg,png,gif`).

### 3. **File Copy Logic**

When the user clicks the "Kopier filer" button, the following happens:
- The program retrieves the paths for the source and target folders from the application state.
- The program splits the `extension_input` string into a vector of extensions (e.g., `["jpg", "png", "gif"]`).
- The program then walks through the source folder (and its subfolders) using the `walkdir` crate.
- For each file, the program checks if its extension matches any of the specified extensions.
- If a match is found, the program copies the file from the source folder to the target folder.

The copying process uses the standard `fs::copy` function to copy the file to the destination.

### 4. **Folder Walking**

- The program uses the `WalkDir` crate to recursively walk through all files and subdirectories in the source folder. The `entry.path()` function returns the full path of each file and directory.
- The `path.extension()` method is used to check the file extension. If the extension matches any of the ones specified by the user, the file is copied to the target folder.

### Example Flow

1. The user clicks the "Velg hovedmappe" button and selects a source folder.
2. The user clicks the "Velg destinasjonsmappe" button and selects a destination folder.
3. The user enters the file extensions to search for in the "Filtyper" input box (e.g., `jpg,png,gif`).
4. The user clicks the "Kopier filer" button, and the program begins scanning the source folder.
5. Files with the specified extensions are copied to the target folder.

## How to Run

1. Clone the repository to your local machine:

   ```bash
   git clone https://github.com/bhelgas/HDD-Checker.git
    ```
2. Navigate to the project directory:
   ```bash
   cd HDD-Checker
   ```
3. Build and run the application using `cargo`:
   ```bash
    cargo run
   ```
4. Follow the on-screen instructions to select the source and target folders and specify the file extensions.

This should now be correctly formatted for your `README.md`. You can just copy and paste this into the markdown file!

