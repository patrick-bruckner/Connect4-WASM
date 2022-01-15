use crate::*;
use tile::Tile;

use yew::prelude::{
    Classes, Component, Callback, ComponentLink, Html, Properties, ShouldRender
};
use yew::macros::{html};

pub struct Col {
    // link: ComponentLink<Self>,
    // click_callback: Callback<usize>,
    click_callback: Callback<yew::events::MouseEvent>,
    style_class: Classes,
    // rows: usize,
    col_data: [Player; ROWS],
    // col_id: usize
}

// pub enum ColMsg {
//     Click
// }

#[derive(Clone, Properties)]
pub struct ColProps {
    // pub click_callback: Callback<usize>,
    pub click_callback: Callback<yew::events::MouseEvent>,
    pub col_data: [Player; ROWS],
    // pub col_id: usize
}

impl Component for Col {
    // type Message = ColMsg;
    type Message = ();
    type Properties = ColProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create("Component",
            format!(r#"
                & {{
                    display: inline-grid;
                    grid-template-rows: repeat({}, auto);
                }}
            "#, ROWS)
        ).unwrap();

        let mut style_class = Classes::new();
        style_class.push(style.get_class_name());

        Self {
            // link,
            click_callback: props.click_callback,
            style_class,
            // rows: ROWS,
            col_data: props.col_data.clone(),
            // col_id: props.col_id
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // match msg {
        //     ColMsg::Click => self.click_callback.emit(self.col_id)
        // };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.col_data != props.col_data {
            self.col_data = props.col_data;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! (
            <div
                class=self.style_class.clone()
                //  onclick=self.link.callback(|_| {ColMsg::Click})>
                onclick=self.click_callback.clone()
            >
            {
                self.col_data.iter().map(|cell| {
                    html! {<Tile player=cell/>}
                }).collect::<Html>()
            }
            </div>
        )
    }
}
