// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! File open/save dialogs.

use std::path::{Path, PathBuf};

/// Information about a file to be opened or saved.
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub(crate) path: PathBuf,
}

/// Type of file dialog.
pub enum FileDialogType {
    /// File open dialog.
    Open,
    /// File save dialog.
    Save,
}

/// Options for file dialogs.
#[derive(Debug, Clone, Default)]
pub struct FileDialogOptions {
    pub show_hidden: bool,
    pub allowed_types: Option<Vec<FileSpec>>,
    // we don't want a library user to be able to construct this type directly
    __non_exhaustive: (),
    // multi selection
    // select directories
}

/// A description of a filetype, for specifiying allowed types in a file dialog.
///
/// # Windows
///
/// On windows, each instance of this type is converted to a [`COMDLG_FILTERSPEC`]
/// struct.
///
/// [`COMDLG_FILTERSPEC`]: https://docs.microsoft.com/en-ca/windows/win32/api/shtypes/ns-shtypes-comdlg_filterspec
#[derive(Debug, Clone, Copy)]
pub struct FileSpec {
    /// A human readable name, describing this filetype.
    ///
    /// This is used in the Windows file dialog, where the user can select
    /// from a dropdown the type of file they would like to choose.
    ///
    /// This should not include the file extensions; they will be added automatically.
    /// For instance, if we are describing Word documents, the name would be "Word Document",
    /// and the displayed string would be "Word Document (*.doc)".
    pub name: &'static str,
    /// The file extensions used by this file type.
    ///
    /// This should not include the leading '.'.
    pub extensions: &'static [&'static str],
}

impl FileInfo {
    /// The file's path.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl FileDialogOptions {
    /// Create a new set of options.
    pub fn new() -> FileDialogOptions {
        FileDialogOptions::default()
    }

    /// Set the 'show hidden files' bit.
    pub fn show_hidden(mut self) -> Self {
        self.show_hidden = true;
        self
    }

    /// Set the filetypes the user is allowed to select.
    pub fn allowed_types(mut self, types: Vec<FileSpec>) -> Self {
        self.allowed_types = Some(types);
        self
    }
}

impl FileSpec {
    pub const TEXT: FileSpec = FileSpec::new("Text", &["txt"]);
    pub const JPG: FileSpec = FileSpec::new("Jpeg", &["jpg", "jpeg"]);
    pub const GIF: FileSpec = FileSpec::new("Gif", &["gif"]);
    pub const PDF: FileSpec = FileSpec::new("PDF", &["pdf"]);
    pub const HTML: FileSpec = FileSpec::new("Web Page", &["htm", "html"]);

    /// Create a new `FileSpec`.
    pub const fn new(name: &'static str, extensions: &'static [&'static str]) -> Self {
        FileSpec { name, extensions }
    }
}
