use std::sync::OnceLock;

use iced::{
  Element, Length,
  alignment::Vertical,
  widget::{Row, container},
};

use super::{Message, State};
use crate::ui::{
  components::{
    button::{Button, Size},
    icon::Icon,
    text_input::TextInput,
  },
  style::{color, spacing},
};

pub(super) fn search_bar(state: &State) -> Element<'_, Message> {
  static PLACEHOLDER: OnceLock<String> = OnceLock::new();
  let placeholder = PLACEHOLDER.get_or_init(|| t!("skills.compare_search.placeholder").into_owned());

  let input = TextInput::new(placeholder, &state.skill_query, Message::SkillQueryChanged)
    .leading_icon(Icon::search())
    .background(color::surface::SUNKEN)
    .render();
  let clear = Button::secondary(t!("skills.compare_search.clear"))
    .icon(Icon::close())
    .size(Size::Sm)
    .on_press(Message::SkillQueryChanged(String::new()))
    .into();

  container(
    Row::with_children(vec![input, clear])
      .spacing(spacing::SPACE_2)
      .align_y(Vertical::Center),
  )
  .width(Length::Fill)
  .padding([spacing::SPACE_2, spacing::SPACE_6])
  .into()
}
