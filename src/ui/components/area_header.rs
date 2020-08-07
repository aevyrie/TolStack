use crate::ui::{components::*, style};
use iced::{
    button, Align, Button, Column, Container, Element, HorizontalAlignment, Length, Row, Text,
};

#[derive(Debug, Clone)]
pub enum Message {
    LabelMessage(editable_label::Message),
    OpenFile,
    SaveFile,
}

#[derive(Debug, Default, Clone)]
pub struct Header {
    pub title: EditableLabel,
    button_open: button::State,
    button_save: button::State,
}
impl Header {
    pub fn new() -> Self {
        Header {
            title: EditableLabel::new("New Project", "Add a project name..."),
            button_open: button::State::new(),
            button_save: button::State::new(),
        }
    }
    pub fn update(&mut self, message: Message) {
        let Header {
            title,
            button_open,
            button_save,
        } = self;
        match message {
            Message::LabelMessage(label_message) => {
                // Pass the message into the title
                title.update(label_message);
            }
            Message::OpenFile => {
                // This message is captured in main.rs
            }
            Message::SaveFile => (
                // This message is captured in main.rs
            ),
        }
    }
    pub fn view(&mut self, iss: &style::IcedStyleSheet) -> Element<Message> {
        let Header {
            title,
            button_open,
            button_save,
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

        let button_open = Button::new(
            button_open,
            Row::new()
                .spacing(iss.spacing(&iss.header_button_internal_spacing))
                .push(Text::new("Open").size(iss.text_size(&iss.header_button_text_size))),
        )
        .on_press(Message::OpenFile)
        .style(iss.button(&iss.header_button_style));

        let button_save = Button::new(
            button_save,
            Row::new()
                .spacing(iss.spacing(&iss.header_button_internal_spacing))
                .push(Text::new("Save").size(iss.text_size(&iss.header_button_text_size))),
        )
        .on_press(Message::SaveFile)
        .style(iss.button(&iss.header_button_style));

        let ribbon = Container::new(
            Row::new()
                .push(button_open)
                .push(button_save)
                .width(Length::Fill)
                .spacing(iss.spacing(&iss.header_button_external_spacing))
        )
        .width(Length::Fill)
        .padding(iss.padding(&iss.header_button_padding))
        .style(iss.container(&iss.header_menu_container));

        let header = Column::new().push(ribbon).push(
            Container::new(
                Column::new()
                    .push(project_title_container)
                    .max_width(800)
            )
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
