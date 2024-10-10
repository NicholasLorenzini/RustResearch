use yew::prelude::*;

struct LoginForm {
    username: String,
    password: String,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    Login,
}

impl Component for LoginForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            username: "".into(),
            password: "".into(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateUsername(new_username) => {
                self.username = new_username;
                true
            }
            Msg::UpdatePassword(new_password) => {
                self.password = new_password;
                true
            }
            Msg::Login => {
                // Handle the login logic here (e.g., API call)
                if !self.username.is_empty() && !self.password.is_empty() {
                    log::info!("Logging in with: {} / {}", self.username, self.password);
                } else {
                    log::warn!("Please fill in both fields");
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let username_oninput = ctx.link().callback(|e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            Msg::UpdateUsername(input.value())
        });

        let password_oninput = ctx.link().callback(|e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            Msg::UpdatePassword(input.value())
        });

        let on_login_click = ctx.link().callback(|_| Msg::Login);

        html! {
            <div>
                <h2>{ "Login" }</h2>
                <div>
                    <label for="username">{ "Username: " }</label>
                    <input id="username" type="text" value={self.username.clone()} oninput={username_oninput} />
                </div>
                <div>
                    <label for="password">{ "Password: " }</label>
                    <input id="password" type="password" value={self.password.clone()} oninput={password_oninput} />
                </div>
                <button onclick={on_login_click}>{ "Login" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<LoginForm>();
}
