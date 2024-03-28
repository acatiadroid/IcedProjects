use iced::highlighter::{self, Highlighter};
use iced::theme;
use iced::widget::{button, column, container, horizontal_space, pick_list, row, text, text_editor, tooltip, TextEditor};
use iced::{executor, Subscription, Application, Command, Element, Font, Length, Settings, Theme};
use iced::keyboard;
use iced;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

fn main() -> iced::Result {
    CryptoDoc::run(
        Settings {
            default_font: Font::MONOSPACE,
            fonts: vec![
                include_bytes!("../fonts/editor-icons.ttf")
                    .as_slice()
                    .into()
            ],
            ..Settings::default()
        }
    )
}

struct CryptoDoc {
    theme: highlighter::Theme,
    path: Option<PathBuf>,
    content: text_editor::Content,
    is_dirty: bool,
}

enum Message {
    ThemeSelected(highlighter::Theme),
    NewDoc,
    DocEdit(text_editor::Action),
    OpenDoc,
    DocOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveDoc,
    DocSaved(Result<PathBuf, Error>),
}

impl Application for CryptoDoc {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                theme: highlighter::Theme::SolarizedDark,
                path: None,
                content: text_editor::Content::new(),
                is_dirty: true,
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("CryptoDoc Document Viewer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NewDoc => {
                self.path = None;
                self.content = text_editor::Content::new();

                Command::none()
            }

            Message::DocEdit(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();
                self.content.edit(action);

                Command::none()
            }

            Message::OpenDoc => Command::perform(pick_file(), Message::DocOpened),
            Message::SaveDoc => {
                let text = self.content.text();

                Command::perform(save_file(self.path.clone(), text), Message::DocSaved)
            }

            Message::DocOpened(Ok((path, content))) => {
                self.path = Some(path);
                self.content = text_editor::Content::with(&content);
                self.is_dirty = false;
            }

            Message::DocOpened(Err(error)) => {
                self.error = Some(error);

                Command::none()
            }

            Message::DocSaved(Ok(path)) => {
                self.path = Some(path);
                self.is_dirty = false;

                Command::none()
            }

            Message::DocSaved(Err(error)) => {
                self.error = Some(error);

                Command::none()
            }

            Message::ThemeSelected(theme) => {
                self.theme = theme;

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = row![
            button(text("New Doc")).on_press(Message::NewDoc),
            button(text("Open Doc")).on_press(Message::OpenDoc),
            button(text("Save Doc")).on_press(Message::SaveDoc),
        ];

        let input = text_editor(&self.content)
            .on_edit(Message::DocEdit)
            .highlight::<Highlighter>(
                highlighter::Settings {
                    theme: self.theme,
                    extension: self
                        .path
                        .as_ref()
                        .and_then(|path| path.extension()?.to_str())
                        .unwrap_or("txt")
                        .to_string(),
                },
                |highlight, _theme| highlight.to_format(),
            );
    }
}

async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose file")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IOFailed)?;

    Ok((path, contents))
}

async fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .set_title("Choose a file name...")
            .save_file()
            .await
            .ok_or(Error::DialogClosed)
            .map(|handle| handle.path().to_owned())?
    };

    tokio::fs::write(&path, text)
        .await
        .map_err(|error| Error::IOFailed(error.kind()))?;

    Ok(path)
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IOFailed(io::ErrorKind),
}
