use yew::prelude::*;

struct LoginForm {
    username: String,
    password: String,
    error_message: Option<String>,
    success_message: Option<String>,
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
            error_message: None,
            success_message: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateUsername(new_username) => {
                self.username = new_username;
                self.error_message = None;  // Clear error message on input change
                self.success_message = None;
                true
            }
            Msg::UpdatePassword(new_password) => {
                self.password = new_password;
                self.error_message = None;  // Clear error message on input change
                self.success_message = None;
                true
            }
            Msg::Login => {
                // Simulate login logic (replace with actual API call in real applications)
                if self.username.is_empty() || self.password.is_empty() {
                    self.error_message = Some("Please fill in both fields.".to_string());
                } else if self.username == "admin" && self.password == "password" {
                    self.success_message = Some("Login successful!".to_string());
                    self.error_message = None;
                } else {
                    self.error_message = Some("Invalid username or password.".to_string());
                    self.success_message = None;
                }
                true // Update the view to display error/success messages
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

                // Display error message if login fails
                if let Some(error) = &self.error_message {
                    html! {
                        <div style="color: red;">{ error }</div>
                    }
                }

                // Display success message if login succeeds
                if let Some(success) = &self.success_message {
                    html! {
                        <div style="color: green;">{ success }</div>
                    }
                }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<LoginForm>();
}
