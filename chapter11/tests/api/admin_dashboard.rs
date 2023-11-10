use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn logout_clears_session_state() {
    let app = spawn_app().await;

    let response = app
        .post_login(&serde_json::json!({
            "username": app.test_user.username,
            "password": &app.test_user.password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    let page_html = app.get_admin_dashboard_html().await;
    assert!(page_html.contains(&format!("Welcome {}", app.test_user.username)));

    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    let page_html = app.get_login_html().await;
    assert!(page_html.contains(r#"<p><i>You have successfully logged out.</i></p>"#));

    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");
}
