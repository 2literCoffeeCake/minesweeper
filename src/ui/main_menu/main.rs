use yew::{Component, Properties, Callback, html, Context, virtual_dom::VNode};
use super::Settings;
pub struct MainMenu{
    state: MenuState

}
pub enum MenuState{
    Main,
    NewGame,
    Settings
}

pub enum Msg{
    SetState(MenuState)
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub start_game: Callback<(usize, usize, usize)>,    
}

impl Component for MainMenu{
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {  
            state: MenuState::Main
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::SetState(state) => self.state = state,
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html!{
            <div class="main_menu">
                <div class="main_menu__title">
                    {self.get_title()}
                </div>
                {self.get_menu_items(ctx)}
            </div>
        }
    }

    
}

impl MainMenu{
    fn get_title(&self) -> String{
        match self.state{
            MenuState::Main => "Minesweeper",
            MenuState::NewGame => todo!(),
            MenuState::Settings => "Settings",
        }.to_string()
    }

    fn get_menu_items(&self, ctx: &yew::Context<Self>)  -> VNode{


        let items = match self.state{
            MenuState::Main => self.get_main_menu_items(ctx),
            MenuState::NewGame => todo!(),
            MenuState::Settings => {
                let back_to_main_menu = ctx.link().callback(|_| Msg::SetState(MenuState::Main));

                return html!{
                    <>
                        <Settings {back_to_main_menu}/>
                    </>
                }

            },
        };
        html!{
            <div class={"main_menu__items"}>
                {items}
            </div>
        }
    }

    fn get_main_menu_items(&self, ctx: &yew::Context<Self>) -> VNode{
        let start_new_game = ctx.link().callback(|_| Msg::SetState(MenuState::NewGame));
        let go_to_settings= ctx.link().callback(|_| Msg::SetState(MenuState::Settings));

        html!{
            <>
                <div onclick={start_new_game}>
                    {"New Game"}
                </div>
                <div onclick={go_to_settings}>
                    {"Settings"}
                </div>
            </>
        }
    }
}