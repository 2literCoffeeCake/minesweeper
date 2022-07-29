use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Clone, Copy)]
pub enum MenuAction {
    Continue,
    BackToMainMenu,
    NewGame,
}

impl ToString for MenuAction {
    fn to_string(&self) -> String {
        let tmp = match self {
            MenuAction::Continue => "Continue",
            MenuAction::BackToMainMenu => "Back to main menu",
            MenuAction::NewGame => "New game",
        };
        String::from(tmp)
    }
}
pub struct Menu {
    actions: Vec<MenuAction>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub on_item_click: Callback<MenuAction>,
    pub active: bool,
    pub continue_button_active: bool,
}

impl Component for Menu {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut vec = Vec::new();
        if ctx.props().clone().continue_button_active {
            vec.push(MenuAction::Continue);
        }
        vec.push(MenuAction::NewGame);
        vec.push(MenuAction::BackToMainMenu);

        Self { actions: vec }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let mut vec = Vec::new();
        if ctx.props().clone().continue_button_active {
            vec.push(MenuAction::Continue);
        }
        vec.push(MenuAction::NewGame);
        vec.push(MenuAction::BackToMainMenu);

        self.actions = vec;
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            title,
            active,
            continue_button_active: _,
            on_item_click: _,
        } = ctx.props().clone();

        let items = self
            .actions
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, action)| {
                let on_item_click = ctx.props().clone().on_item_click;
                let on_click = ctx.link().callback(move|_| on_item_click.emit(action));
                return html! {
                    <>
                        <div style={format!("grid-row: {}/{};font-size: 50px;", (4 + (2*index)).to_string(), (5 + (2*index)).to_string())} class="menu_menu_item" onclick={on_click}>{action.to_string()}</div>
                    </>
                };
            })
            .collect::<Html>();

        let classes = get_classes(active);
        html! {
            <div class={classes}>
                <div style={"grid-row: 2/3;font-size: 90px;"} class="menu_menu_item">{title}</div>
                {items}
            </div>
        }
    }
}

fn get_classes(active: bool) -> String {
    let mut classes = String::from("menu ");
    if active {
        classes.push_str("menu--active");
    } else {
        classes.push_str("menu--inactive");
    }
    classes
}
