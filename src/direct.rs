//! Direct mouse and keyboard manipulation.
use crate::{elements::Element, Client};
use webdriver::actions::KeyAction;
use webdriver::actions::KeyActionItem;
use webdriver::actions::KeyDownAction;
use webdriver::actions::KeyUpAction;
use webdriver::actions::PointerAction;
use webdriver::actions::PointerActionItem;
use webdriver::actions::PointerActionParameters;
use webdriver::actions::PointerDownAction;
use webdriver::actions::PointerMoveAction;
use webdriver::actions::PointerUpAction;
use webdriver::actions::{ActionSequence, ActionsType, PointerOrigin};
use webdriver::command::{ActionsParameters, WebDriverCommand};

/// Mouse event variants.
#[derive(Debug)]
pub enum MouseEvent {
    /// Move event.
    Move(MouseMoveEvent),
    /// Button event.
    Button(MouseButtonEvent),
}

/// Mouse move event.
#[derive(Debug)]
pub struct MouseMoveEvent {
    /// x position relative to the root element, or the document.
    pub x: i64,
    /// y position relative to the root element, or the document.
    pub y: i64,
}

/// Mouse button event.
#[derive(Debug)]
pub struct MouseButtonEvent {
    /// If the button was pressed down as part of this event.
    /// If this is false, the button was released as part of this event.
    pub pressed: bool,
    /// The button being pressed.
    pub button: MouseButton,
}

/// Mouse button variants.
#[derive(Debug)]
pub enum MouseButton {
    /// Primary button, generally the left mouse button.
    Primary,
    /// Secondary button, generally the right mouse button.
    Secondary,
}

impl MouseButton {
    fn to_number(&self) -> u64 {
        match self {
            Self::Primary => 0,
            Self::Secondary => 2,
        }
    }
}

/// Keyboard event.
#[derive(Debug)]
pub struct KeyboardEvent {
    /// If the key was pressed or released as part of this event.
    pub pressed: bool,
    /// The value of the key being pressed or released.
    pub value: String,
}

/// Direct mouse and keyboard manipulation.
impl Client {
    /// TODO
    pub async fn apply_mouse_events(
        &mut self,
        root: Option<Element>,
        events: Vec<MouseEvent>,
    ) -> Result<(), crate::error::CmdError> {
        let actions = events
            .into_iter()
            .map(|ev| Self::make_mouse_action(root.as_ref(), ev))
            .collect();
        // TODO (JAB): Can make this an argument.
        let parameters = PointerActionParameters {
            pointer_type: webdriver::actions::PointerType::Mouse,
        };

        let action = ActionsType::Pointer {
            parameters,
            actions,
        };

        let action = ActionSequence {
            id: "default mouse".to_string(),
            actions: action,
        };

        let action = ActionsParameters {
            actions: vec![action],
        };

        let cmd = WebDriverCommand::PerformActions(action);
        self.issue(cmd).await?;
        Ok(())
    }

    /// TODO
    pub async fn apply_keyboard_events(
        &mut self,
        events: Vec<KeyboardEvent>,
    ) -> Result<(), crate::error::CmdError> {
        let actions: Vec<_> = events.into_iter().map(Self::make_key_action).collect();

        let action = ActionsType::Key { actions };
        let action = ActionSequence {
            id: "default keyboard".to_string(),
            actions: action,
        };

        let action = ActionsParameters {
            actions: vec![action],
        };

        let cmd = WebDriverCommand::PerformActions(action);
        self.issue(cmd).await?;
        Ok(())
    }

    fn make_mouse_action(root: Option<&Element>, event: MouseEvent) -> PointerActionItem {
        let origin = match root {
            None => PointerOrigin::Viewport,
            Some(r) => PointerOrigin::Element(r.element.clone()),
        };
        let action = match event {
            MouseEvent::Move(ev) => {
                let action = PointerMoveAction {
                    duration: None,
                    origin,
                    x: Some(ev.x),
                    y: Some(ev.y),
                };

                PointerAction::Move(action)
            }
            MouseEvent::Button(ev) => {
                let button = ev.button.to_number();
                if ev.pressed {
                    PointerAction::Down(PointerDownAction { button })
                } else {
                    PointerAction::Up(PointerUpAction { button })
                }
            }
        };

        PointerActionItem::Pointer(action)
    }

    fn make_key_action(event: KeyboardEvent) -> KeyActionItem {
        let action = if event.pressed {
            KeyAction::Down(KeyDownAction { value: event.value })
        } else {
            KeyAction::Up(KeyUpAction { value: event.value })
        };

        KeyActionItem::Key(action)
    }
}
