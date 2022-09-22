use yew::{html, Callback, Component, Context, Properties, Html};

#[derive(Clone, PartialEq, Properties)]
pub struct Props{
    pub back_to_main_menu: Callback<()>,
    pub start_game: Callback<(usize, usize, usize)>
}

pub enum Msg{
    BackToMainMenu,
    StartGame(usize, usize, usize)
}

pub struct NewGameMenu {
    levels: Vec<DifficultyLevel>,
}

impl Component for NewGameMenu {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let levels: Vec<DifficultyLevel> = Vec::from([
            DifficultyLevel::easy(),
            DifficultyLevel::middle(),
            DifficultyLevel::hard(),
        ]);

        Self { levels }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Props {back_to_main_menu, start_game} = ctx.props().clone();

        match msg{
            Msg::BackToMainMenu => back_to_main_menu.emit(()),
            Msg::StartGame(r, c, a) => start_game.emit((r, c, a)),
        }



        true
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {

        let back_to_main_menu = ctx.link().callback(move |_| Msg::BackToMainMenu);

        let items = self.levels.clone().into_iter().enumerate().map(|(index, level)|{
            let new_game = ctx.link().callback(move |_| Msg::StartGame(level.row, level.column, level.amount_bombs));

            html!{
                <>
                    <div style={format!("grid-row: {}/{}; grid-column:1/2", index + 1, index + 2)} onclick={new_game}>{level.name}</div>
                </>
            }

        }).collect::<Html>();


        html!{
            <div class="main_menu__new_game">
                {items}
                <div style={format!("grid-row: 4/5; grid-column:1/2")} onclick={back_to_main_menu}>{"Back to main menu"}</div>
            </div>
        }
    }
}


#[derive(Clone, PartialEq)]
struct DifficultyLevel {
    name: String,
    row: usize,
    column: usize,
    amount_bombs: usize,
}

impl DifficultyLevel {
    fn easy() -> Self {
        Self {
            name: "Easy".to_owned(),
            row: 8,
            column: 6,
            amount_bombs: 8,
        }
    }

    fn middle() -> Self {
        Self {
            name: "Middle".to_owned(),
            row: 12,
            column: 9,
            amount_bombs: 16,
        }
    }

    fn hard() -> Self {
        Self {
            name: "Hard".to_owned(),
            row: 18,
            column: 14,
            amount_bombs: 32,
        }
    }


}

