use crate::analysis::structures::*;
use crate::ui::{icons, style};
use iced::{
    button, text_input, Align, Button, Checkbox, Column, Container, Element, HorizontalAlignment,
    Length, Row, Text, TextInput,
};
use serde_derive::*;

#[derive(Debug, Clone)]
pub enum State {
    Idle {
        button_edit: button::State,
        button_move_up: button::State,
        button_move_down: button::State,
    },
    Editing {
        form_tolentry: Box<FormState>,
    },
}
impl Default for State {
    fn default() -> Self {
        State::Idle {
            button_edit: button::State::new(),
            button_move_up: button::State::new(),
            button_move_down: button::State::new(),
        }
    }
}

#[allow(clippy::clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum FormState {
    Linear {
        button_save: button::State,
        button_delete: button::State,
        description: text_input::State,
        dimension: text_input::State,
        tolerance_pos: text_input::State,
        tolerance_neg: text_input::State,
        sigma: text_input::State,
    },
    Float {
        button_save: button::State,
        button_delete: button::State,
        description: text_input::State,
        diameter_hole: text_input::State,
        diameter_pin: text_input::State,
        tolerance_hole_pos: text_input::State,
        tolerance_hole_neg: text_input::State,
        tolerance_pin_pos: text_input::State,
        tolerance_pin_neg: text_input::State,
        sigma: text_input::State,
    },
}
impl FormState {
    pub fn new(form_type: Tolerance) -> Self {
        match form_type {
            Tolerance::Linear(_) => FormState::Linear {
                button_save: button::State::new(),
                button_delete: button::State::new(),
                description: text_input::State::new(),
                dimension: text_input::State::new(),
                tolerance_pos: text_input::State::new(),
                tolerance_neg: text_input::State::new(),
                sigma: text_input::State::new(),
            },
            Tolerance::Float(_) => FormState::Float {
                button_save: button::State::new(),
                button_delete: button::State::new(),
                description: text_input::State::new(),
                diameter_hole: text_input::State::new(),
                diameter_pin: text_input::State::new(),
                tolerance_hole_pos: text_input::State::new(),
                tolerance_hole_neg: text_input::State::new(),
                tolerance_pin_pos: text_input::State::new(),
                tolerance_pin_neg: text_input::State::new(),
                sigma: text_input::State::new(),
            },
        }
    }
    pub fn new_focused(form_type: Tolerance) -> Self {
        match form_type {
            Tolerance::Linear(_) => FormState::Linear {
                button_save: button::State::new(),
                button_delete: button::State::new(),
                description: text_input::State::focused(),
                dimension: text_input::State::new(),
                tolerance_pos: text_input::State::new(),
                tolerance_neg: text_input::State::new(),
                sigma: text_input::State::new(),
            },
            Tolerance::Float(_) => FormState::Float {
                button_save: button::State::new(),
                button_delete: button::State::new(),
                description: text_input::State::focused(),
                diameter_hole: text_input::State::new(),
                diameter_pin: text_input::State::new(),
                tolerance_hole_pos: text_input::State::new(),
                tolerance_hole_neg: text_input::State::new(),
                tolerance_pin_pos: text_input::State::new(),
                tolerance_pin_neg: text_input::State::new(),
                sigma: text_input::State::new(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    // Entry messages
    EntryActive(bool),
    EntryEdit,
    EntryDelete,
    EntryFinishEditing,
    EntryMoveUp,
    EntryMoveDown,
    // Shared Field messages
    EditedDescription(String),
    // Linear entry messages
    EditedLinearDimension(String),
    EditedLinearTolerancePos(String),
    EditedLinearToleranceNeg(String),
    EditedLinearSigma(String),
    // Float entry messages
    EditedFloatDiameterHole(String),
    EditedFloatDiameterPin(String),
    EditedFloatTolHolePos(String),
    EditedFloatTolHoleNeg(String),
    EditedFloatTolPinPos(String),
    EditedFloatTolPinNeg(String),
    EditedFloatSigma(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormValues {
    Linear {
        description: String,
        dimension: String,
        tolerance_pos: String,
        tolerance_neg: String,
        sigma: String,
    },
    Float {
        description: String,
        diameter_hole: String,
        diameter_pin: String,
        tolerance_hole_pos: String,
        tolerance_hole_neg: String,
        tolerance_pin_pos: String,
        tolerance_pin_neg: String,
        sigma: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToleranceEntry {
    pub input: FormValues,
    pub analysis_model: Tolerance,
    pub active: bool,
    pub valid: bool,

    #[serde(skip)]
    pub state: State,
}
impl ToleranceEntry {
    pub fn new(description: String, tolerance: Tolerance) -> Self {
        ToleranceEntry {
            input: match tolerance {
                Tolerance::Linear(_) => FormValues::Linear {
                    description,
                    dimension: String::from(""),
                    tolerance_pos: String::from(""),
                    tolerance_neg: String::from(""),
                    sigma: String::from(""),
                },
                Tolerance::Float(_) => FormValues::Float {
                    description,
                    diameter_hole: String::from(""),
                    diameter_pin: String::from(""),
                    tolerance_hole_pos: String::from(""),
                    tolerance_hole_neg: String::from(""),
                    tolerance_pin_pos: String::from(""),
                    tolerance_pin_neg: String::from(""),
                    sigma: String::from(""),
                },
            },
            analysis_model: tolerance,
            active: false,
            valid: false,
            state: State::default(),
        }
    }

    pub fn with_editing(mut self) -> Self {
        self.state = State::Editing {
            form_tolentry: Box::new(FormState::new(self.analysis_model)),
        };
        self
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::EntryActive(is_active) => {
                if self.valid {
                    self.active = is_active
                } else {
                    self.active = false;
                }
            }
            Message::EntryEdit => {
                self.state = State::Editing {
                    form_tolentry: Box::new(FormState::new_focused(self.analysis_model)),
                };
            }
            Message::EntryFinishEditing => {
                if match &self.input {
                    FormValues::Linear { description, .. } => !description.is_empty(),
                    FormValues::Float { description, .. } => !description.is_empty(),
                } {
                    self.state = State::default()
                }
            }
            Message::EntryDelete => {}
            Message::EntryMoveUp => {}
            Message::EntryMoveDown => {}
            Message::EditedDescription(input) => {
                match &mut self.input {
                    FormValues::Linear { description, .. } => *description = input,
                    FormValues::Float { description, .. } => *description = input,
                };
            }
            Message::EditedLinearDimension(input) => {
                if let FormValues::Linear { dimension, .. } = &mut self.input {
                    *dimension = NumericString::eval(dimension, &input, NumericString::Number)
                };
            }
            Message::EditedLinearTolerancePos(input) => {
                if let FormValues::Linear { tolerance_pos, .. } = &mut self.input {
                    *tolerance_pos =
                        NumericString::eval(tolerance_pos, &input, NumericString::Positive)
                };
            }
            Message::EditedLinearToleranceNeg(input) => {
                if let FormValues::Linear { tolerance_neg, .. } = &mut self.input {
                    *tolerance_neg =
                        NumericString::eval(tolerance_neg, &input, NumericString::Positive)
                };
            }
            Message::EditedLinearSigma(input) => {
                if let FormValues::Linear { sigma, .. } = &mut self.input {
                    *sigma = NumericString::eval(sigma, &input, NumericString::Positive)
                };
            }
            Message::EditedFloatDiameterHole(input) => {
                if let FormValues::Float { diameter_hole, .. } = &mut self.input {
                    *diameter_hole =
                        NumericString::eval(diameter_hole, &input, NumericString::Positive)
                };
            }
            Message::EditedFloatDiameterPin(input) => {
                if let FormValues::Float { diameter_pin, .. } = &mut self.input {
                    *diameter_pin =
                        NumericString::eval(diameter_pin, &input, NumericString::Positive)
                };
            }
            Message::EditedFloatTolHolePos(input) => {
                if let FormValues::Float {
                    tolerance_hole_pos, ..
                } = &mut self.input
                {
                    *tolerance_hole_pos =
                        NumericString::eval(tolerance_hole_pos, &input, NumericString::Positive)
                };
            }
            Message::EditedFloatTolHoleNeg(input) => {
                if let FormValues::Float {
                    tolerance_hole_neg, ..
                } = &mut self.input
                {
                    *tolerance_hole_neg =
                        NumericString::eval(tolerance_hole_neg, &input, NumericString::Positive)
                };
            }
            Message::EditedFloatTolPinPos(input) => {
                if let FormValues::Float {
                    tolerance_pin_pos, ..
                } = &mut self.input
                {
                    *tolerance_pin_pos =
                        NumericString::eval(tolerance_pin_pos, &input, NumericString::Positive)
                };
            }
            Message::EditedFloatTolPinNeg(input) => {
                if let FormValues::Float {
                    tolerance_pin_neg, ..
                } = &mut self.input
                {
                    *tolerance_pin_neg =
                        NumericString::eval(tolerance_pin_neg, &input, NumericString::Positive)
                };
            }
            Message::EditedFloatSigma(input) => {
                if let FormValues::Float { sigma, .. } = &mut self.input {
                    *sigma = NumericString::eval(sigma, &input, NumericString::Positive)
                };
            }
        }
    }

    pub fn view(&mut self, iss: &style::IcedStyleSheet) -> Element<Message> {
        match &mut self.state {
            State::Idle {
                button_edit,
                button_move_up,
                button_move_down,
            } => {
                let checkbox = Checkbox::new(
                    self.active,
                    match &self.input {
                        FormValues::Linear { description, .. } => description,
                        FormValues::Float { description, .. } => description,
                    },
                    Message::EntryActive,
                )
                .width(Length::Fill);

                let summary = Text::new(match self.valid {
                    true => match self.analysis_model {
                        Tolerance::Linear(dim) => {
                            if (dim.distance.tol_neg - dim.distance.tol_pos).abs() < f64::EPSILON {
                                format!("{} +/- {}", dim.distance.dim, dim.distance.tol_pos)
                            } else {
                                format!(
                                    "{} +{}/-{}",
                                    dim.distance.dim, dim.distance.tol_pos, dim.distance.tol_neg
                                )
                            }
                        }
                        Tolerance::Float(dim) => {
                            let hole = if (dim.hole.tol_neg - dim.hole.tol_pos).abs() < f64::EPSILON
                            {
                                format!("{} +/- {}", dim.hole.dim, dim.hole.tol_pos)
                            } else {
                                format!(
                                    "{} +{}/-{}",
                                    dim.hole.dim, dim.hole.tol_pos, dim.hole.tol_neg
                                )
                            };
                            let pin = if (dim.pin.tol_neg - dim.pin.tol_pos).abs() < f64::EPSILON {
                                format!("{} +/- {}", dim.pin.dim, dim.pin.tol_pos)
                            } else {
                                format!("{} +{}/-{}", dim.pin.dim, dim.pin.tol_pos, dim.pin.tol_neg)
                            };
                            format!("Hole: {}\nPin: {}", hole, pin)
                        }
                    },
                    false => "Incomplete entry".to_string(),
                })
                .size(iss.text_size(&iss.tol_entry_summary_text_size));

                let edit_button = Button::new(
                    button_edit,
                    Row::new()
                        //.push(Text::new("Edit")
                        //    .size(iss.text_size(&iss.tol_entry_button_text_size)))
                        .push(icons::edit().size(iss.text_size(&iss.tol_entry_button_text_size)))
                        .spacing(iss.spacing(&iss.tol_entry_button_spacing)),
                )
                .on_press(Message::EntryEdit)
                .padding(iss.padding(&iss.tol_entry_button_padding))
                .style(iss.button(&iss.button_action));

                let up_button = Button::new(
                    button_move_up,
                    Row::new()
                        //.push(Text::new("Edit")
                        //    .size(iss.text_size(&iss.tol_entry_button_text_size)))
                        .push(
                            icons::up_arrow().size(iss.text_size(&iss.tol_entry_button_text_size)),
                        )
                        .spacing(iss.spacing(&iss.tol_entry_button_spacing)),
                )
                .on_press(Message::EntryMoveUp)
                .padding(iss.padding(&iss.tol_entry_button_padding))
                .style(iss.button(&iss.button_inactive));

                let down_button = Button::new(
                    button_move_down,
                    Row::new()
                        //.push(Text::new("Edit")
                        //    .size(iss.text_size(&iss.tol_entry_button_text_size)))
                        .push(
                            icons::down_arrow()
                                .size(iss.text_size(&iss.tol_entry_button_text_size)),
                        )
                        .spacing(iss.spacing(&iss.tol_entry_button_spacing)),
                )
                .on_press(Message::EntryMoveDown)
                .padding(iss.padding(&iss.tol_entry_button_padding))
                .style(iss.button(&iss.button_inactive));

                let row_contents = Row::new()
                    .padding(iss.padding(&iss.tol_entry_padding))
                    .spacing(iss.spacing(&iss.tol_entry_spacing))
                    .align_items(Align::Center)
                    .push(checkbox)
                    .push(summary)
                    .push(edit_button)
                    .push(up_button)
                    .push(down_button);

                //Container::new(row_contents)
                //    .style(iss.container(&iss.tol_entry_container))
                //    .into()

                row_contents.into()
            }
            State::Editing { form_tolentry } => match &mut **form_tolentry {
                FormState::Linear {
                    button_save,
                    button_delete,
                    description,
                    dimension,
                    tolerance_pos,
                    tolerance_neg,
                    sigma,
                } => {
                    let view_button_save = Button::new(
                        button_save,
                        Row::new()
                            .spacing(10)
                            .push(icons::check())
                            .push(Text::new("Save")),
                    )
                    .on_press(Message::EntryFinishEditing)
                    .padding(10)
                    .style(iss.button(&iss.button_constructive));

                    let view_button_delete = Button::new(
                        button_delete,
                        Row::new()
                            .spacing(10)
                            .push(icons::delete())
                            .push(Text::new("Delete")),
                    )
                    .on_press(Message::EntryDelete)
                    .padding(10)
                    .style(iss.button(&iss.button_destructive));

                    let view_description = TextInput::new(
                        description,
                        "Enter a description",
                        match &self.input {
                            FormValues::Linear { description, .. } => description,
                            _ => "Error: tolerance type mismatch",
                        },
                        Message::EditedDescription,
                    )
                    .on_submit(Message::EntryFinishEditing)
                    .padding(iss.padding(&iss.tol_edit_field_padding))
                    .size(iss.text_size(&iss.tol_edit_field_text_size));

                    let view_dimension = TextInput::new(
                        dimension,
                        "Enter a value",
                        match &self.input {
                            FormValues::Linear { dimension, .. } => dimension,
                            _ => "Error: tolerance type mismatch",
                        },
                        Message::EditedLinearDimension,
                    )
                    .on_submit(Message::EntryFinishEditing)
                    .padding(iss.padding(&iss.tol_edit_field_padding))
                    .size(iss.text_size(&iss.tol_edit_field_text_size));

                    let view_tolerance_pos = TextInput::new(
                        tolerance_pos,
                        "Enter a value",
                        match &self.input {
                            FormValues::Linear { tolerance_pos, .. } => tolerance_pos,
                            _ => "Error: tolerance type mismatch",
                        },
                        Message::EditedLinearTolerancePos,
                    )
                    .on_submit(Message::EntryFinishEditing)
                    .padding(iss.padding(&iss.tol_edit_field_padding))
                    .size(iss.text_size(&iss.tol_edit_field_text_size));

                    let view_tolerance_neg = TextInput::new(
                        tolerance_neg,
                        "Enter a value",
                        match &self.input {
                            FormValues::Linear { tolerance_neg, .. } => tolerance_neg,
                            _ => "Error: tolerance type mismatch",
                        },
                        Message::EditedLinearToleranceNeg,
                    )
                    .on_submit(Message::EntryFinishEditing)
                    .padding(iss.padding(&iss.tol_edit_field_padding))
                    .size(iss.text_size(&iss.tol_edit_field_text_size));

                    let view_sigma = TextInput::new(
                        sigma,
                        "Enter a value",
                        match &self.input {
                            FormValues::Linear { sigma, .. } => sigma,
                            _ => "Error: tolerance type mismatch",
                        },
                        Message::EditedLinearSigma,
                    )
                    .on_submit(Message::EntryFinishEditing)
                    .padding(iss.padding(&iss.tol_edit_field_padding))
                    .size(iss.text_size(&iss.tol_edit_field_text_size));

                    let row_header = Row::new()
                        .push(
                            Text::new("Editing Linear Tolerance")
                                .size(iss.text_size(&iss.tol_edit_heading_text_size))
                                .width(Length::Fill)
                                .horizontal_alignment(HorizontalAlignment::Left),
                        )
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_description = Row::new()
                        .push(Column::new().width(Length::Units(20)))
                        .push(
                            Text::new("Description:")
                                .size(iss.text_size(&iss.tol_edit_label_text_size)),
                        )
                        .push(view_description)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_dimension = Row::new()
                        .push(Column::new().width(Length::Units(20)))
                        .push(
                            Text::new("Dimension:")
                                .size(iss.text_size(&iss.tol_edit_label_text_size)),
                        )
                        .push(view_dimension)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_tolerance_pos = Row::new()
                        .push(Column::new().width(Length::Units(20)))
                        .push(
                            Text::new("+ Tolerance:")
                                .size(iss.text_size(&iss.tol_edit_label_text_size)),
                        )
                        .push(view_tolerance_pos)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_tolerance_neg = Row::new()
                        .push(Column::new().width(Length::Units(20)))
                        .push(
                            Text::new("- Tolerance:")
                                .size(iss.text_size(&iss.tol_edit_label_text_size)),
                        )
                        .push(view_tolerance_neg)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_sigma = Row::new()
                        .push(Column::new().width(Length::Units(20)))
                        .push(
                            Text::new("Sigma:").size(iss.text_size(&iss.tol_edit_label_text_size)),
                        )
                        .push(view_sigma)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_buttons = Row::new()
                        .push(view_button_delete)
                        .push(view_button_save)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let entry_contents = Column::new()
                        .push(row_header)
                        .push(Row::new().height(Length::Units(5)))
                        .push(row_description)
                        .push(row_dimension)
                        .push(row_tolerance_pos)
                        .push(row_tolerance_neg)
                        .push(row_sigma)
                        .push(Row::new().height(Length::Units(5)))
                        .push(row_buttons)
                        .spacing(iss.spacing(&iss.tol_edit_vspacing))
                        .padding(iss.padding(&iss.tol_edit_padding));

                    Container::new(entry_contents)
                        .style(iss.container(&iss.tol_entry_container))
                        .into()
                }
                FormState::Float {
                    button_save,
                    button_delete,
                    description,
                    diameter_hole,
                    diameter_pin,
                    tolerance_hole_pos,
                    tolerance_hole_neg,
                    tolerance_pin_pos,
                    tolerance_pin_neg,
                    sigma,
                } => {
                    let view_button_save = Button::new(
                        button_save,
                        Row::new()
                            .spacing(10)
                            .push(icons::check())
                            .push(Text::new("Save")),
                    )
                    .on_press(Message::EntryFinishEditing)
                    .padding(10)
                    .style(iss.button(&iss.button_constructive));

                    let view_button_delete = Button::new(
                        button_delete,
                        Row::new()
                            .spacing(10)
                            .push(icons::delete())
                            .push(Text::new("Delete")),
                    )
                    .on_press(Message::EntryDelete)
                    .padding(10)
                    .style(iss.button(&iss.button_destructive));

                    let view_description = TextInput::new(
                        description,
                        "Enter a description",
                        match &self.input {
                            FormValues::Float { description, .. } => description,
                            _ => "Error: tolerance type mismatch",
                        },
                        Message::EditedDescription,
                    )
                    .on_submit(Message::EntryFinishEditing)
                    .padding(iss.padding(&iss.tol_edit_field_padding))
                    .size(iss.text_size(&iss.tol_edit_field_text_size));

                    let view_diameter_hole = TextInput::new(
                        diameter_hole,
                        "Enter a value",
                        match &self.input {
                            FormValues::Float { diameter_hole, .. } => diameter_hole,
                            _ => "Error: tolerance type mismatch",
                        },
                        Message::EditedFloatDiameterHole,
                    )
                    .on_submit(Message::EntryFinishEditing)
                    .padding(iss.padding(&iss.tol_edit_field_padding))
                    .size(iss.text_size(&iss.tol_edit_field_text_size));

                    let view_diameter_pin = TextInput::new(
                        diameter_pin,
                        "Enter a value",
                        match &self.input {
                            FormValues::Float { diameter_pin, .. } => diameter_pin,
                            _ => "Error: tolerance type mismatch",
                        },
                        Message::EditedFloatDiameterPin,
                    )
                    .on_submit(Message::EntryFinishEditing)
                    .padding(iss.padding(&iss.tol_edit_field_padding))
                    .size(iss.text_size(&iss.tol_edit_field_text_size));

                    let view_tolerance_hole_pos = TextInput::new(
                        tolerance_hole_pos,
                        "Enter a value",
                        match &self.input {
                            FormValues::Float {
                                tolerance_hole_pos, ..
                            } => tolerance_hole_pos,
                            _ => "Error: tolerance type mismatch",
                        },
                        Message::EditedFloatTolHolePos,
                    )
                    .on_submit(Message::EntryFinishEditing)
                    .padding(iss.padding(&iss.tol_edit_field_padding))
                    .size(iss.text_size(&iss.tol_edit_field_text_size));

                    let view_tolerance_hole_neg = TextInput::new(
                        tolerance_hole_neg,
                        "Enter a value",
                        match &self.input {
                            FormValues::Float {
                                tolerance_hole_neg, ..
                            } => tolerance_hole_neg,
                            _ => "Error: tolerance type mismatch",
                        },
                        Message::EditedFloatTolHoleNeg,
                    )
                    .on_submit(Message::EntryFinishEditing)
                    .padding(iss.padding(&iss.tol_edit_field_padding))
                    .size(iss.text_size(&iss.tol_edit_field_text_size));

                    let view_tolerance_pin_pos = TextInput::new(
                        tolerance_pin_pos,
                        "Enter a value",
                        match &self.input {
                            FormValues::Float {
                                tolerance_pin_pos, ..
                            } => tolerance_pin_pos,
                            _ => "Error: tolerance type mismatch",
                        },
                        Message::EditedFloatTolPinPos,
                    )
                    .on_submit(Message::EntryFinishEditing)
                    .padding(iss.padding(&iss.tol_edit_field_padding))
                    .size(iss.text_size(&iss.tol_edit_field_text_size));

                    let view_tolerance_pin_neg = TextInput::new(
                        tolerance_pin_neg,
                        "Enter a value",
                        match &self.input {
                            FormValues::Float {
                                tolerance_pin_neg, ..
                            } => tolerance_pin_neg,
                            _ => "Error: tolerance type mismatch",
                        },
                        Message::EditedFloatTolPinNeg,
                    )
                    .on_submit(Message::EntryFinishEditing)
                    .padding(iss.padding(&iss.tol_edit_field_padding))
                    .size(iss.text_size(&iss.tol_edit_field_text_size));

                    let view_sigma = TextInput::new(
                        sigma,
                        "Enter a value",
                        match &self.input {
                            FormValues::Float { sigma, .. } => sigma,
                            _ => "Error: tolerance type mismatch",
                        },
                        Message::EditedFloatSigma,
                    )
                    .on_submit(Message::EntryFinishEditing)
                    .padding(iss.padding(&iss.tol_edit_field_padding))
                    .size(iss.text_size(&iss.tol_edit_field_text_size));

                    let row_header = Row::new()
                        .push(
                            Text::new("Editing Float Tolerance")
                                .size(iss.text_size(&iss.tol_edit_heading_text_size))
                                .width(Length::Fill)
                                .horizontal_alignment(HorizontalAlignment::Left),
                        )
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_description = Row::new()
                        .push(Column::new().width(Length::Units(20)))
                        .push(
                            Text::new("Description:")
                                .size(iss.text_size(&iss.tol_edit_label_text_size)),
                        )
                        .push(view_description)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_diameter_hole = Row::new()
                        .push(Column::new().width(Length::Units(20)))
                        .push(
                            Text::new("Hole Diameter:")
                                .size(iss.text_size(&iss.tol_edit_label_text_size)),
                        )
                        .push(view_diameter_hole)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_diameter_pin = Row::new()
                        .push(Column::new().width(Length::Units(20)))
                        .push(
                            Text::new("Pin Diameter:")
                                .size(iss.text_size(&iss.tol_edit_label_text_size)),
                        )
                        .push(view_diameter_pin)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_tolerance_hole_pos = Row::new()
                        .push(Column::new().width(Length::Units(20)))
                        .push(
                            Text::new("+ Hole Tolerance:")
                                .size(iss.text_size(&iss.tol_edit_label_text_size)),
                        )
                        .push(view_tolerance_hole_pos)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_tolerance_hole_neg = Row::new()
                        .push(Column::new().width(Length::Units(20)))
                        .push(
                            Text::new("- Hole Tolerance:")
                                .size(iss.text_size(&iss.tol_edit_label_text_size)),
                        )
                        .push(view_tolerance_hole_neg)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_tolerance_pin_pos = Row::new()
                        .push(Column::new().width(Length::Units(20)))
                        .push(
                            Text::new("+ Pin Tolerance:")
                                .size(iss.text_size(&iss.tol_edit_label_text_size)),
                        )
                        .push(view_tolerance_pin_pos)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_tolerance_pin_neg = Row::new()
                        .push(Column::new().width(Length::Units(20)))
                        .push(
                            Text::new("- Pin Tolerance:")
                                .size(iss.text_size(&iss.tol_edit_label_text_size)),
                        )
                        .push(view_tolerance_pin_neg)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_sigma = Row::new()
                        .push(Column::new().width(Length::Units(20)))
                        .push(
                            Text::new("Sigma:").size(iss.text_size(&iss.tol_edit_label_text_size)),
                        )
                        .push(view_sigma)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let row_buttons = Row::new()
                        .push(view_button_delete)
                        .push(view_button_save)
                        .spacing(iss.spacing(&iss.tol_edit_label_spacing))
                        .align_items(Align::Center);

                    let entry_contents = Column::new()
                        .push(row_header)
                        .push(Row::new().height(Length::Units(5)))
                        .push(row_description)
                        .push(Text::new("Hole Dimensions"))
                        .push(row_diameter_hole)
                        .push(row_tolerance_hole_pos)
                        .push(row_tolerance_hole_neg)
                        .push(Text::new("Pin Dimensions"))
                        .push(row_diameter_pin)
                        .push(row_tolerance_pin_pos)
                        .push(row_tolerance_pin_neg)
                        .push(row_sigma)
                        .push(Row::new().height(Length::Units(5)))
                        .push(row_buttons)
                        .spacing(iss.spacing(&iss.tol_edit_vspacing))
                        .padding(iss.padding(&iss.tol_edit_padding));

                    Container::new(entry_contents)
                        .style(iss.container(&iss.tol_entry_container))
                        .into()
                }
            },
        }
    }
}

enum NumericString {
    Number,
    Positive,
    //Negative,
}
impl NumericString {
    pub fn eval(old: &str, input: &str, criteria: Self) -> String {
        match input.parse::<f64>().is_ok() {
            true => {
                let numeric_input = input.parse::<f64>().unwrap();
                if match criteria {
                    NumericString::Number => true,
                    NumericString::Positive => numeric_input >= 0.0,
                    //NumericString::Negative => numeric_input < 0.0,
                } || input.is_empty()
                    || input == "."
                {
                    input.to_string()
                } else {
                    old.to_string()
                }
            }
            false => {
                if match criteria {
                    NumericString::Number => input.is_empty() || input == "-" || input == ".",
                    //NumericString::Negative => input == "" || input == "-" || input == ".",
                    NumericString::Positive => input.is_empty() || input == ".",
                } {
                    input.to_string()
                } else {
                    old.to_string()
                }
            }
        }
    }
}
