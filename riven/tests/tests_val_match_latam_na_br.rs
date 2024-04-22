mod testutils;
use riven::{consts::*, models::val_match_v1::MatchlistEntry};
use testutils::{riot_api, riven_test, val_content_ranked, val_match_v1_get, val_match_v1_latest};

const ROUTE: ValPlatformRoute = ValPlatformRoute::NA;

static MATCHES: &[&str] = &[
    "b979be9e-aeb1-419e-981d-d7d2472649a2",
    "c086129c-e538-43cf-ae93-7148d0d01182",
];

#[riven_test]
async fn val_match_v1_get_test() -> Result<(), String> {
    val_match_v1_get(ROUTE, MATCHES).await
}

#[riven_test]
async fn val_content_ranked_test() -> Result<(), String> {
    val_content_ranked(ROUTE).await
}

#[riven_test]
async fn val_match_v1_latest_test() -> Result<(), String> {
    val_match_v1_latest(ROUTE).await
}

#[riven_test]
async fn val_match_v1_get_matchlist_theuscon() -> Result<(), String> {
    let account = riot_api()
        .account_v1()
        .get_by_riot_id(RegionalRoute::AMERICAS, "Theusçon", "8119")
        .await
        .map_err(|e| format!("Failed to get account: {}", e))?
        .ok_or_else(|| "Account not found!".to_owned())?;

    let matchlist = riot_api()
        .val_match_v1()
        .get_matchlist(ROUTE, &account.puuid)
        .await
        .map_err(|e| format!("Failed to get matchlist: {}", e))?;

    val_match_v1_get(ROUTE, matchlist.history.into_iter().map(|entry| entry.match_id)).await
}
