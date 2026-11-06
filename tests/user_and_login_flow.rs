#[cfg(test)]
mod test {
    use std::net::TcpListener;

    
    use playwright_rs::{GetByRoleOptions, Playwright, expect};
    use sqlx::SqlitePool;

    fn spawn_app(pool: SqlitePool) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        // We retrieve the port assigned to us by the OS
        let port = listener.local_addr().unwrap().port();
        let server = homequest::app::run(listener, pool, None).expect("Failed to bind address");
        let _ = tokio::spawn(server);
        format!("http://127.0.0.1:{}", port)
    }

    #[sqlx::test]
    async fn create_user_account(pool: SqlitePool) -> Result<(), playwright_rs::Error> {
        let appaddress = spawn_app(pool);
        let pw = Playwright::launch().await?;
        let browser = pw.chromium().launch().await?;
        let page = browser.new_page().await?;

        page.goto(appaddress.as_str(), None).await?;

        let create_link = page.get_by_text("Create account", true);
        create_link.click(None).await?;

        let username = page.locator("[name='username']");
        username.fill("foo", None).await?;
        let displayname = page.locator("[name='displayname']");
        displayname.fill("Foobar", None).await?;
        let password = page.locator("[name='password']");
        password.fill("123456", None).await?;
        let password_repeat = page.locator("#password_confirm");
        password_repeat.fill("123456", None).await?;
        let submit_button = page.get_by_role(
            playwright_rs::AriaRole::Button,
            Some(GetByRoleOptions::default().name("Create")),
        );

        submit_button.click(None).await?;

        expect(page.get_by_text("Logout", false).first())
            .to_be_visible()
            .await?;
        assert_eq!(page.url(), appaddress + "/");

        browser.close().await?;
        Ok(())
    }

    #[sqlx::test]
    async fn login_as_user(pool: SqlitePool) -> Result<(), playwright_rs::Error> {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
        let credentials = homequest::testing::create_test_admin_account(pool.clone()).await;

        let appaddress = spawn_app(pool);
        let pw = Playwright::launch().await?;
        let browser = pw.chromium().launch().await?;
        let page = browser.new_page().await?;

        page.goto(appaddress.as_str(), None).await?;

        let username = page.locator("[name='username']");
        username.fill(&credentials.username, None).await?;
        let password = page.locator("[name='password']");
        password.fill(&credentials.password, None).await?;
        let submit_button = page.get_by_role(
            playwright_rs::AriaRole::Button,
            Some(GetByRoleOptions::default().name("Login")),
        );

        submit_button.click(None).await?;

        expect(page.get_by_text("Logout", false).first())
            .to_be_visible()
            .await?;
        assert_eq!(page.url(), appaddress + "/");
        
        browser.close().await?;
        Ok(())
    }
}
