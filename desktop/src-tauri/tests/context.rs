#[cfg(test)]
mod common;

#[cfg(test)]
mod tests {
    use tauri::Manager;
    use thunes_cli::{
        account::{Account, AddAccountOptions},
        context::Context,
    };
    use thunes_lib::commands::{
        account::add_account,
        context::{get_context, update_context},
    };

    async fn setup() -> (tauri::App<tauri::test::MockRuntime>, Account) {
        let app = crate::common::setup().await;
        let account = add_account(
            app.state(),
            AddAccountOptions {
                currency: "EUR".to_string(),
                name: "My Account".to_string(),
            },
        )
        .await
        .expect("failed to create account");

        (app, account)
    }

    #[tokio::test]
    pub async fn test_get_context() {
        let (app, _account) = setup().await;

        let context = get_context(app.state())
            .await
            .expect("failed to get context");

        assert_eq!(context.last_opened_account, None);
    }

    #[tokio::test]
    pub async fn test_update_context() {
        let (app, account) = setup().await;

        update_context(
            app.state(),
            Context {
                last_opened_account: Some(account.id.clone()),
                last_opened_budget: None,
            },
        )
        .await
        .expect("failed to update context");
        let context = get_context(app.state())
            .await
            .expect("failed to get context");

        assert_eq!(context.last_opened_account, Some(account.id));
        assert_eq!(context.last_opened_budget, None);
    }
}
