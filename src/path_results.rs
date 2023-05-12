use std::borrow::Borrow;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use gtk::prelude::*;

use crate::search_results::ListItem;
use crate::search_results::Results;

#[allow(dead_code)]
pub struct PathResults {
    child_paths: Vec<PathBuf>,
}

impl PathResults {
    pub fn new(search_query: &str) -> Self {
        let child_paths: Vec<PathBuf> = fs::read_dir(search_query).map_or(Vec::new(), |entries| {
            entries.map(|entry| entry.unwrap().path()).collect()
        });
        Self { child_paths }
    }
}

impl Results for PathResults {
    fn is_empty(&self) -> bool {
        self.child_paths.is_empty()
    }

    fn create_list_items(&self) -> Vec<ListItem> {
        self.child_paths
            .iter()
            .map(|child_path| create_list_item(child_path))
            .collect()
    }

    fn on_item_selected(&self, item: &ListItem) {
        let Some(child_path) = self.child_paths.get(item.index() as usize) else {
            return;
        };
        if let Some(basename) = child_path.file_name() {
            let basename = basename.to_string_lossy().to_string();
            item.activate_action("win.change-query", Some(&basename.to_variant()))
                .expect("action `change-query` should exist");
        }
    }
}

fn create_list_item(child_path: &Path) -> ListItem {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    container.append(&create_icon_widget(child_path));

    let path_name = child_path.file_name().unwrap_or_default().to_string_lossy();
    container.append(&create_label_widget(&path_name));
    ListItem::builder().child(&container).build()
}

fn create_icon_widget(path: &Path) -> gtk::Image {
    let icon = gtk::Image::from_icon_name(get_icon_name_from_path(path));
    icon.set_margin_top(2);
    icon.set_margin_bottom(2);
    icon.set_margin_start(10);
    icon.set_margin_end(2);
    icon.set_pixel_size(25);
    icon
}

fn get_icon_name_from_path(path: &Path) -> &str {
    if path.is_dir() {
        return "folder";
    }

    let Some(file_extension) = path.extension() else {
        return "application-x-executable";
    };

    match file_extension.to_string_lossy().borrow() {
        // application
        "default" => "application-octet-stream",
        "abw" => "application-x-abiword",
        "arc" => "application-x-freearc",
        "azw" => "application-vnd.amazon.ebook",
        "bin" => "application-octet-stream",
        "bz" => "application-x-bzip",
        "bz2" => "application-x-bzip2",
        "cda" => "application-x-cdf",
        "csh" => "application-x-csh",
        "doc" => "application-msword",
        "docx" => "application-vnd.openxmlformats-officedocument.wordprocessingml.document",
        "eot" => "application-vnd.ms-fontobject",
        "epub" => "application-epub+zip",
        "gz" => "application-gzip",
        "jar" => "application-java-archive",
        "json" => "application-json",
        "jsonld" => "application-ld+json",
        "mpkg" => "application-vnd.apple.installer+xml",
        "odp" => "application-vnd.oasis.opendocument.presentation",
        "ods" => "application-vnd.oasis.opendocument.spreadsheet",
        "odt" => "application-vnd.oasis.opendocument.text",
        "ogx" => "application-ogg",
        "pdf" => "application-pdf",
        "php" => "application-x-httpd-php",
        "ppt" => "application-vnd.ms-powerpoint",
        "pptx" => "application-vnd.openxmlformats-officedocument.presentationml.presentation",
        "rar" => "application-vnd.rar",
        "rtf" => "application-rtf",
        "sh" => "application-x-sh",
        "tar" => "application-x-tar",
        "vsd" => "application-vnd.visio",
        "xhtml" => "application-xhtml+xml",
        "xls" => "application-vnd.ms-excel",
        "xlsx" => "application-vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "xml" => "application-xml",
        "xul" => "application-vnd.mozilla.xul+xml",
        "zip" => "application-zip",
        "7z" => "application-x-7z-compressed",

        // audio
        "aac" => "audio-aac",
        "mid" => "audio-midi",
        "midi" => "audio-x-midi",
        "mp3" => "audio-mpeg",
        "oga" => "audio-ogg",
        "opus" => "audio-opus",
        "wav" => "audio-wav",
        "weba" => "audio-webm",

        // font
        "otf" => "font-otf",
        "woff" => "font-woff",
        "woff2" => "font-woff2",

        // image
        "avif" => "image-avif",
        "bmp" => "image-bmp",
        "gif" => "image-gif",
        "ico" => "image-vnd.microsoft.icon",
        "jpeg" | "jpg" => "image-jpeg",
        "png" => "image-png",
        "svg" => "image-svg+xml",
        "tif" | "tiff" => "image-tiff",
        "webp" => "image-webp",

        // text
        "css" => "text-css",
        "csv" => "text-csv",
        "htm" | "html" => "text-html",
        "ics" => "text-calendar",
        "js" | "mjs" => "text-javascript",
        "txt" => "text-plain",

        // video
        "avi" => "video-x-msvideo",
        "mkv" => "video-x-generic",
        "mp4" => "video-mp4",
        "mpeg" => "video-mpeg",
        "ogv" => "video-ogg",
        "ts" | "ttf" => "video-mp2t",
        "webm" => "video-webm",
        "3gp" => "video-3gpp",
        "3g2" => "video-3gpp2",

        _ => "unknown",
    }
}

fn create_label_widget(path_name: &str) -> gtk::Label {
    let name = gtk::Label::new(Some(path_name));
    name.set_halign(gtk::Align::Start);
    name
}
