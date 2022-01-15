use crate::*;

use yew::prelude::{
    Classes, Component, ComponentLink, Html, Properties, ShouldRender
};
use yew::macros::{classes, html};

pub struct Tile {
    style_class: Classes,
    player: Player
}

#[derive(Clone, Properties)]
pub struct TileProps {
    pub player: Player
}

impl Component for Tile {
    type Message = ();
    type Properties = TileProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = Style::create("Component",
            r#"
                & {
                    height: 50px;
                    width: 50px;
                    border: 1px solid black;
                }
                &.Red {
                    background: red;
                }
                &.Blue {
                    background: blue;
                }
            "#
        ).unwrap();

        let mut style_class = Classes::new();
        style_class.push(style.get_class_name());

        Self {
            style_class,
            player: props.player
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.player != props.player {
            self.player = props.player;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! (
            // <tile class=self.style_class.clone()>
            <div class=classes!(self.player.to_string(), self.style_class.clone())/>
            // </tile>
        )
    }
}
