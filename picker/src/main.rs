use iced::widget::{column, pick_list, scrollable, vertical_space};
use iced::{Alignment, Element, Length};

pub fn main() -> iced::Result {
    iced::run("Pick List", Example::update, Example::view)
}

#[derive(Default)]
struct Example {
    selected_language: Option<Language>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    LanguageSelected(Language),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::LanguageSelected(language) => {
                self.selected_language = Some(language);

                println!("{}", self.selected_language);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let pick_list = pick_list(
            &Language::ALL[..],
            self.selected_language,
            Message::LanguageSelected
        ).placeholder("Choose a language");

        let content = column![
            vertical_space().height(600),
            "Pick your favourite language:",
            pick_list,
            vertical_space().height(600),
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10);

        scrollable(content).into()

    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    Rust,
    Elm,
    Ruby,
    Haskell,
    C,
    JavaScript,
    Other,
}

impl Language {
    const ALL: [Language; 7] = [
        Language::C,
        Language::Elm,
        Language::Ruby,
        Language::Haskell,
        Language::Rust,
        Language::JavaScript,
        Language::Other,
    ];
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Rust => "Rust",
                Language::Elm => "Elm",
                Language::Ruby => "Ruby",
                Language::Haskell => "Haskell",
                Language::C => "C",
                Language::JavaScript => "JavaScript",
                Language::Other => "Other language",
            }
        )
    }
}