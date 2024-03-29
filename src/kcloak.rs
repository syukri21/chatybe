use core::panic;
use dotenvy::dotenv;
use futures::TryFutureExt;
use keycloak::{types::RealmRepresentation, KeycloakAdmin, KeycloakAdminToken};
use std::env;

#[warn(dead_code)]
pub struct KeyCloakImpl {
    url: String,
    user: String,
    password: String,
    kcloak: KeycloakAdmin,
}

impl KeyCloakImpl {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let url = env::var("KEYCLOAK_URL").expect("KEYCLOAK_URL must be set");
        let user = env::var("KEYCLOAK_ADMIN").expect("KEYCLOAK_ADMIN must be set");
        let password =
            env::var("KEYCLOAK_ADMIN_PASSWORD").expect("KEYCLOAK_ADMIN_PASSWORD must be set");
        let realm = "chaty";
        let client = reqwest::Client::new();
        let admin_token = KeycloakAdminToken::acquire(&url, &user, &password, &client).await?;

        tracing::info!("got admin token");
        tracing::debug!("{:#?}", admin_token);

        let admin = KeycloakAdmin::new(&url, admin_token, client);

        let realm = admin
            .realm_get(realm)
            .or_else(|_| async {
                admin
                    .post(RealmRepresentation {
                        realm: Some(realm.to_string()),
                        ..Default::default()
                    })
                    .await
                    .expect("could not create realm");
                admin.realm_get(realm).await
            })
            .await?;

        let rname = realm.realm.expect("no realm name found");

        Ok(Self {
            url,
            user,
            password,
            kcloak: admin,
        })
    }
}
