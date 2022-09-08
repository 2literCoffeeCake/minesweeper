use super::{Minefield, Button, Menu, MenuAction};
use yew::{html, Component, Context, Html, Properties, virtual_dom::VNode, Callback};
use uuid::Uuid;

#[derive(Debug)]
pub struct Playground{
    state: GameState,
    game_id: Uuid
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameState{
    Playing,
    GameOver,
    Pausing,
    Success
}

impl ToString for GameState {
    fn to_string(&self) -> String {
        match self {
            GameState::Playing => "Playing",
            GameState::GameOver => "Game over",
            GameState::Pausing => "Pausing",
            GameState::Success => "Success",
        }.to_string()
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub amount_bombs: usize,
    pub rows: usize,
    pub columns: usize,

    pub back_to_menu: Callback<()>,    
}

pub enum Msg{
    SetPlayingState(GameState),
    NewGame,
    BackToMainMenu
}

impl Component for Playground {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            state: GameState::Playing,
            game_id: Uuid::new_v4(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let back_to_menu = ctx.props().clone().back_to_menu;

        match msg{
            Msg::SetPlayingState(state) => self.state = state,
            Msg::NewGame => {
                self.state = GameState::Playing;
                self.game_id = Uuid::new_v4();
            },
            Msg::BackToMainMenu => back_to_menu.emit(()),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            rows,
            columns,
            amount_bombs,
            back_to_menu: _
        } = ctx.props().clone();

        let on_bomb_click = ctx.link().callback(|_| Msg::SetPlayingState(GameState::GameOver));
        let on_game_win = ctx.link().callback(|_| Msg::SetPlayingState(GameState::Success));

        let menu = self.render_menu(ctx);
        let buttons = self.render_buttons(ctx);

        html! {
            <div class="game">
                <div class="playground">
                    {buttons}
                    <Minefield {columns} {rows} {amount_bombs} {on_bomb_click} game_id={self.game_id} {on_game_win}/>
                </div>
                {menu}
            </div>
        }
    }
}

impl Playground{
    fn render_menu(&self, ctx: &Context<Self>) -> VNode{
        let on_item_click = ctx.link().callback(|menu_action| {
            match menu_action{
                MenuAction::Continue => Msg::SetPlayingState(GameState::Playing),
                MenuAction::BackToMainMenu => Msg::BackToMainMenu,
                MenuAction::NewGame => Msg::NewGame,
            }
        });

        let active = self.state != GameState::Playing;
        let continue_button_active = self.state == GameState::Pausing;
        let title = match self.state{
            GameState::Playing => "",
            GameState::GameOver => "Game over",
            GameState::Pausing => "Pause",
            GameState::Success => "You've won",
        };

        html!{
            <>
                <Menu {title} {on_item_click} {active} {continue_button_active}/>
            </>
        }
    }

    fn render_buttons(&self, ctx: &Context<Self>) -> VNode{
        let on_back_button_click = ctx.link().callback(|_| Msg::SetPlayingState(GameState::Pausing));
        let on_menu_button_click = ctx.link().callback(|_| Msg::SetPlayingState(GameState::Pausing));

        html!{
            <>
                <Button on_click={on_back_button_click} row={2}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                    <path d="M272 431.952v-73.798h128c26.51 0 48-21.49 48-48V201.846c0-26.51-21.49-48-48-48H272V80.057c0-42.638-51.731-64.15-81.941-33.941l-176 175.943c-18.745 18.745-18.746 49.137 0 67.882l176 175.952C220.208 496.042 272 474.675 272 431.952zM48 256L224 80v121.846h176v108.308H224V432L48 256z"/>
                </svg>
                </Button>
                <Button on_click={on_menu_button_click} row={4}>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                        <path d="M16 132h416c8.837 0 16-7.163 16-16V76c0-8.837-7.163-16-16-16H16C7.163 60 0 67.163 0 76v40c0 8.837 7.163 16 16 16zm0 160h416c8.837 0 16-7.163 16-16v-40c0-8.837-7.163-16-16-16H16c-8.837 0-16 7.163-16 16v40c0 8.837 7.163 16 16 16zm0 160h416c8.837 0 16-7.163 16-16v-40c0-8.837-7.163-16-16-16H16c-8.837 0-16 7.163-16 16v40c0 8.837 7.163 16 16 16z"/>
                    </svg>
                </Button>
            </>
        }
    }
}
