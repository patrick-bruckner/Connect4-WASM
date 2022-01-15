#![recursion_limit="256"]
mod player;
use player::Player;

mod tile;
mod col;
use col::Col;

use yew::prelude::{
    Classes, Component, ComponentLink, Html, ShouldRender
};
use yew::macros::{html};
use css_in_rust::Style;
use wasm_bindgen::prelude::wasm_bindgen;

const COLS: usize = 7;
const ROWS: usize = 6;

struct App {
    link: ComponentLink<Self>,
    style_class: Classes,
    board: [[Player; ROWS]; COLS],
    player: Player,
    winner_declared: bool
}

enum AppMsg {
    ColClick(usize)
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create("Component",
            r#"
                p {
                    left: 50%;
                    transform: translate(0, -50%);
                    text-align: center;
                }
                .board {
                    display: grid;
                    justify-content: center;
                    grid-template-columns: repeat(var(--cols), min-content);
                }
            "#
        ).unwrap();
        let mut style_class = Classes::new();
        style_class.push(style.get_class_name());
        Self {
            link,
            style_class,
            board: [[Player::None; ROWS]; COLS],
            player: Player::Blue,
            winner_declared: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::ColClick(i) => {
                if !self.winner_declared {
                    if self.add_chip_to_col(i) {
                        if self.check_for_winner() {
                            self.winner_declared = true;
                        } else {
                            self.chnge_player();
                        }
                        return true
                    }
                }
            }
        }
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! (
            <app class=self.style_class.clone()>
                <p>{"Wellcome to Connect Four!"}</p>
                {
                    if self.winner_declared {
                        html! {<p>{format!("{} wins!", self.player)}</p>}
                    } else {
                        html! {<p>{format!("{}'s turn", self.player)}</p>}
                    }
                }
                <div class="board" style=format!("--cols: {}", COLS)>
                {
                    self.board.iter().enumerate().map(|(idx, col)| {
                        html! {
                                <Col
                                    // click_callback=self.link.callback(|i| AppMsg::ColClick(i))
                                    click_callback=self.link.callback(move |_| AppMsg::ColClick(idx))
                                    // col_id=idx
                                    col_data=col.clone()
                                />
                            }
                    }).collect::<Html>()
                }
                </div>
            </app>
        )
    }
}

impl App {
    fn chnge_player(&mut self) {
        self.player = if self.player == Player::Blue {
            Player::Red
        } else {
            Player::Blue
        };
    }

    fn add_chip_to_col(&mut self, col_idx: usize) -> bool {
        for cell in &mut self.board[col_idx].iter_mut().rev() {
            if *cell == Player::None {
                *cell = self.player;
                return true;
            }
        }

        false
    }

    fn check_for_winner(&self) -> bool {
        let mut line_found = false;

        for col in 0..self.board.len() {
            for row in 0..self.board[col].len() {
                line_found |= self.check_for_winner_inner(col as i32, row as i32,
                                                          0, |c,r| (c+1,r));
                line_found |= self.check_for_winner_inner(col as i32, row as i32,
                                                          0, |c,r| (c,r-1));
                line_found |= self.check_for_winner_inner(col as i32, row as i32,
                                                          0, |c,r| (c+1,r-1));
                line_found |= self.check_for_winner_inner(col as i32, row as i32,
                                                          0, |c,r| (c-1,r-1));

                if line_found { break; }
            }
        }

        line_found
    }

    fn check_for_winner_inner<F>(&self, col: i32,
                                 row: i32, count: usize,
                                 move_fn: F) -> bool
                                    where F: Fn(i32, i32) -> (i32, i32) {
        if count == 4 {
            return true;
        }

        if (col < 0) ||
           (row < 0) ||
           (col >= (self.board.len() as i32)) ||
           (row >= (self.board[col as usize].len() as i32)) {
            return false;
        }

        if self.board[col as usize][row as usize] != self.player {
            return false;
        }

        let (new_col, new_row) = move_fn(col, row);

        return self.check_for_winner_inner(new_col, new_row, count+1, move_fn);
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::start_app::<App>();
}
