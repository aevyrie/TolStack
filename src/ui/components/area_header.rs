use crate::ui::{components::*, icons, style};
use iced::{
    button, Align, Button, Column, Container, Element, HorizontalAlignment, Length, Row, Text,
    VerticalAlignment,
};
use iced_native::Renderer;

#[derive(Debug, Clone)]
pub enum Message {
    LabelMessage(editable_label::Message),
    NewFile,
    OpenFile,
    SaveFile,
    SaveAsFile,
    ExportCSV,
}

#[derive(Debug, Default, Clone)]
pub struct Header {
    pub title: EditableLabel,
    button_new: button::State,
    button_open: button::State,
    button_save: button::State,
    button_export: button::State,
    button_save_as: button::State,
}
impl Header {
    pub fn new() -> Self {
        Header {
            title: EditableLabel::new("New Project", "Add a project name..."),
            button_new: button::State::new(),
            button_open: button::State::new(),
            button_save: button::State::new(),
            button_export: button::State::new(),
            button_save_as: button::State::new(),
        }
    }
    pub fn update(&mut self, message: Message) {
        let Header {
            title,
            button_new,
            button_open,
            button_save,
            button_export,
            button_save_as,
        } = self;
        match message {
            Message::LabelMessage(label_message) => {
                // Pass the message into the title
                title.update(label_message);
            }
            Message::NewFile => (),   // This message is captured in main.rs
            Message::OpenFile => (),  // This message is captured in main.rs
            Message::SaveFile => (),  // This message is captured in main.rs
            Message::SaveAsFile => (), // This message is captured in main.rs
            Message::ExportCSV => (), // This message is captured in main.rs
        }
    }
    pub fn view(&mut self, iss: &style::IcedStyleSheet) -> Element<Message> {
        let Header {
            title,
            button_new,
            button_open,
            button_save,
            button_export,
            button_save_as
        } = self;
        let project_label = Text::new("Project: ")
            .width(Length::Shrink)
            .size(iss.text_size(&iss.project_label_text_size))
            .color(iss.color(&iss.project_label_color))
            .horizontal_alignment(HorizontalAlignment::Left);

        let project_title: Row<_> = Row::new()
            .push(project_label)
            .push(
                title
                    .view(&iss)
                    .map(move |message| Message::LabelMessage(message)),
            )
            .align_items(Align::Center)
            .spacing(iss.spacing(&iss.project_label_spacing))
            .into();

        let project_title_container =
            Container::new(Row::new().push(project_title).width(Length::Shrink))
                .width(Length::Fill)
                .center_x()
                .center_y();

        let button_new =
            header_button(button_new, "New\n", icons::new(), iss).on_press(Message::NewFile);

        let button_open =
            header_button(button_open, "Open\n", icons::load(), iss).on_press(Message::OpenFile);

        let button_save =
            header_button(button_save, "Save\n", icons::save(), iss).on_press(Message::SaveFile);

        let button_save_as =
            header_button(button_save_as, "Save As\n", icons::save(), iss).on_press(Message::SaveAsFile);

        let button_export = header_button(button_export, "Export CSV", icons::export(), iss)
            .on_press(Message::ExportCSV);

        let ribbon = Container::new(
            Row::new()
                .push(button_new)
                .push(button_open)
                .push(button_save)
                .push(button_save_as)
                .push(button_export)
                .width(Length::Fill)
                .spacing(iss.spacing(&iss.header_button_external_spacing)),
        )
        .width(Length::Fill)
        .padding(iss.padding(&iss.header_button_padding))
        .style(iss.container(&iss.header_menu_container));

        let header = Column::new()
            .push(ribbon)
            .push(
                Container::new(Column::new().push(project_title_container).max_width(800))
                    .width(Length::Fill)
                    .padding(10)
                    .center_x(),
            )
            .spacing(iss.spacing(&iss.header_spacing));

        header.into()
    }
    pub fn title(&mut self, title: String) -> Self {
        self.title.text = title;
        self.clone()
    }
}

fn header_button<'a>(
    state: &'a mut button::State,
    text: &str,
    icon: Text,
    iss: &style::IcedStyleSheet,
) -> Button<'a, Message> {
    Button::new(
        state,
        Column::new()
            .spacing(iss.spacing(&iss.header_button_internal_spacing))
            .push(
                Container::new(icon.size(iss.text_size(&iss.header_button_icon_size)))
                    .center_x()
                    .center_y()
                    //.height(Length::Fill)
                    .width(Length::Fill),
            )
            .push(
                Container::new(
                    Text::new(text)
                        .width(Length::Fill)
                        .size(iss.text_size(&iss.header_button_text_size))
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .center_x()
                .center_y()
                .width(Length::Fill),
            ),
    )
    .style(iss.button(&iss.header_button_style))
    .height(iss.dimension(&iss.header_button_height))
    .width(iss.dimension(&iss.header_button_width))
}
