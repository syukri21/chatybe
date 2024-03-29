use dotenvy::dotenv;
use futures::TryFutureExt;
use keycloak::{types::RealmRepresentation, KeycloakAdmin, KeycloakAdminToken};
use shaku::{Component, Interface};
use std::env;

#[warn(dead_code)]
#[derive(Component)]
#[shaku(interface = Auth)]
pub struct KCloak {
    kcloak: KeycloakAdmin,
}

pub trait Auth: Interface {
    fn get_keycloak_admin(&self) -> &KeycloakAdmin;
    fn get_user(&self) -> String;
}

impl KCloak {
    pub async fn new() -> KCloak {
        dotenv().ok();
        let url = env::var("KEYCLOAK_URL").expect("KEYCLOAK_URL must be set");
        let user = env::var("KEYCLOAK_ADMIN").expect("KEYCLOAK_ADMIN must be set");
        let password =
            env::var("KEYCLOAK_ADMIN_PASSWORD").expect("KEYCLOAK_ADMIN_PASSWORD must be set");
        let realm = "chaty";
        let client = reqwest::Client::new();
        let kcloak_client_token = KeycloakAdminToken::acquire(&url, &user, &password, &client)
            .await
            .expect("Error acquire token");
        tracing::info!("got kcloak_client token");

        let kcloak_client = KeycloakAdmin::new(&url, kcloak_client_token, client);
        tracing::info!("instantiated keycloak");
        // Create realm chaty if not exist
        kcloak_client
            .realm_get(realm)
            .or_else(|_| async {
                kcloak_client
                    .post(RealmRepresentation {
                        realm: Some(realm.to_string()),
                        ..Default::default()
                    })
                    .await
                    .expect("could not create realm");
                kcloak_client.realm_get(realm).await
            })
            .await
            .expect("Error while creating realm");

        Self {
            kcloak: kcloak_client,
        }
    }
}

impl Auth for KCloak {
    fn get_keycloak_admin(&self) -> &KeycloakAdmin {
        &self.kcloak
    }
    fn get_user(&self) -> String {
        String::from("kcloak_client")
    }
}
