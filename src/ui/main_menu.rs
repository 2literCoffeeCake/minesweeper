use yew::{Component, Properties, Callback, html, Context, virtual_dom::VNode};

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
            MenuState::Settings => todo!(),
        }.to_string()
    }

    fn get_menu_items(&self, ctx: &yew::Context<Self>)  -> VNode{
        let items = match self.state{
            MenuState::Main => self.get_main_menu_items(ctx),
            MenuState::NewGame => todo!(),
            MenuState::Settings => todo!(),
        };
        html!{
            <div class={"main_menu__items"}>
                {items}
            </div>
        }
    }

    fn get_main_menu_items(&self, _ctx: &yew::Context<Self>) -> VNode{
        html!{
            <>
                <div>
                    {"New Game"}
                </div>
                <div>
                    {"Settings"}
                </div>
            </>
        }
    }
}