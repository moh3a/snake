mod random;
mod snake;

use snake::{Direction, SnakeGame};

use js_sys::Function;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{console, window, HtmlDivElement, HtmlElement, KeyboardEvent};

thread_local! {
    static GAME: Rc<RefCell<SnakeGame>> = Rc::new(RefCell::new(SnakeGame::new(20, 20)));

    static TICK_HANDLER: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        || {
            GAME.with(|game| game.borrow_mut().tick());
            render();
        }
    }) as Box<dyn FnMut()>);

    static KEYDOWN_HANDLER: Closure<dyn FnMut(KeyboardEvent)> = Closure::wrap(Box::new({
        |event: KeyboardEvent| {
            GAME.with(|game| {
                let direction =  match &event.key()[..] {
                    "ArrowUp" => Some(Direction::Up),
                    "ArrowDown" => Some(Direction::Down),
                    "ArrowRight" => Some(Direction::Right),
                    "ArrowLeft" => Some(Direction::Left),
                    _ => None,
                };

                if let Some(direction) = direction {
                    game.borrow_mut().change_direction(direction);
                }
            }
        );
        }
    }) as Box<dyn FnMut(KeyboardEvent)>)
}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Starting...".into());

    TICK_HANDLER.with(|tick| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                tick.as_ref().dyn_ref::<Function>().unwrap_throw(),
                200,
            )
            .unwrap_throw()
    });

    KEYDOWN_HANDLER.with(|event| {
        window()
            .unwrap_throw()
            .add_event_listener_with_callback(
                "keydown",
                event.as_ref().dyn_ref::<Function>().unwrap_throw(),
            )
            .unwrap_throw();
    });

    render();
}

pub fn render() {
    GAME.with(|game| {
        let document = window().unwrap_throw().document().unwrap_throw();

        let root_container = document
            .get_element_by_id("root")
            .unwrap_throw()
            .dyn_into::<HtmlElement>()
            .unwrap_throw();

        root_container.set_inner_html("");

        let width = game.borrow().width;
        let height = game.borrow().height;

        root_container
            .style()
            .set_property("display", "inline-grid")
            .unwrap_throw();
        root_container
            .style()
            .set_property(
                "grid-template",
                &format!("repeat({}, auto) / repeat({}, auto)", height, width),
            )
            .unwrap_throw();

        for y in 0..height {
            for x in 0..width {
                let pos = (x, y);
                let field_element = document
                    .create_element("div")
                    .unwrap_throw()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap_throw();

                field_element.set_class_name("field");

                field_element.set_inner_text({
                    if pos == game.borrow().food {
                        "🥐"
                    } else if game.borrow().snake.get(0) == Some(&pos) {
                        "✳️"
                    } else if game.borrow().snake.contains(&pos) {
                        "🟩"
                    } else {
                        " "
                    }
                });

                root_container.append_child(&field_element).unwrap_throw();
            }
        }
    });
}
