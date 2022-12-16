use std::fmt::Display;

use iced::{Application, Command, Element, Settings, Alignment, Length};
use iced::executor;
use iced::theme::Theme;
use iced::widget::{TextInput, Button, Text, Column, Row};
fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res: &str;
        match self {
            Operation::Add => res = "+",
            Operation::Subtract => res = "-",
            Operation::Multiply => res = "*",
            Operation::Divide => res = "/",
        }
        write!(f, "{}", res)
    }
}
struct Calculator {
    first_value: f32,
    second_value: f32,
    operation: Option<Operation>,
    result: f32,
}

#[derive(Debug, Clone)]
enum Message {
    EnterFirstValue(f32),
    EnterSecondValue(f32),
    Calculate(Operation),
}

impl Application for Calculator {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                first_value: 0.0,
                second_value: 0.0,
                operation: None,
                result: 0.0,
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        "Calculator".to_owned()
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::EnterFirstValue(val) => self.first_value = val,
            Message::EnterSecondValue(val) => self.second_value = val,
            Message::Calculate(op) => {
                match op {
                    Operation::Add => self.result = self.first_value + self.second_value,
                    Operation::Subtract => self.result = self.first_value - self.second_value,
                    Operation::Multiply => self.result = self.first_value * self.second_value,
                    Operation::Divide => self.result = self.first_value / self.second_value,
                }
                self.operation = Some(op);
            },
        }
        Command::none()
    }
    fn view(&self) -> Element<Message> {
        let screen = 
        screen(&self.first_value, &self.second_value, &self.operation, &self.result);
        let num_inputs =
        num_inputs(&self.first_value, &self.second_value);
        let op_buttons =
        op_buttons();
        Column::new()
        .push(screen)
        .push(num_inputs)
        .push(op_buttons)
        .padding(10)
        .spacing(20)
        .align_items(Alignment::Center)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}

fn screen<'a>(first_val: &f32, second_val: &f32, operation: &Option<Operation>, res: &f32) -> Element<'a, Message> {
    let current_op = if operation.is_none() {"".to_owned()} else {operation.unwrap().to_string()};
    let op_text: Text = 
    Text::new(format!("{} {} {}", first_val, current_op, second_val))
    .size(20);
    let res_text: Text =
    Text::new(res.to_string());
    Column::new().push(op_text).push(res_text).into()
}

fn num_inputs<'a>(first_val: &f32, second_val: &f32) -> Element<'a, Message> {
    let first_field: TextInput<Message> = 
    TextInput::new("Input first number", &first_val.to_string(), 
    |input| Message::EnterFirstValue(input.parse::<f32>().unwrap_or_default()));
    let second_field: TextInput<Message> =
    TextInput::new("Input second number", &second_val.to_string(),
    |input| Message::EnterSecondValue(input.parse::<f32>().unwrap_or_default()));

    Row::new().push(first_field).push(second_field).into()
}

fn op_buttons<'a>() -> Element<'a, Message> {
    let add_button = 
    Button::new("+")
    .on_press(Message::Calculate(Operation::Add))
    .width(Length::Fill);

    let sub_button = 
    Button::new("-")
    .on_press(Message::Calculate(Operation::Subtract))
    .width(Length::Fill);

    let mul_button = 
    Button::new("*")
    .on_press(Message::Calculate(Operation::Multiply))
    .width(Length::Fill);

    let div_button = 
    Button::new("/")
    .on_press(Message::Calculate(Operation::Divide))
    .width(Length::Fill);

    let first_row = Row::new()
    .push(add_button)
    .push(sub_button)
    .padding(50)
    .spacing(10)
    .align_items(Alignment::Center);

    let second_row = Row::new()
    .push(mul_button)
    .push(div_button)
    .padding(50)
    .spacing(10)
    .align_items(Alignment::Center);

    Column::new()
    .push(first_row)
    .push(second_row)
    .spacing(30)
    .into()
}