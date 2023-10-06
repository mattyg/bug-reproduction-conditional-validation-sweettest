use hdk::prelude::*;
use holochain::{conductor::{config::ConductorConfig, api::error::ConductorApiError}, sweettest::*, prelude::DnaFile};
use posts_integrity::{Post, DnaProperties};
use holochain::test_utils::consistency_60s;

pub async fn load_dna() -> DnaFile {
    // Use prebuilt dna file
    let dna_path = std::env::current_dir()
      .unwrap()
      .join("../../../workdir/demo.dna");
    
    SweetDnaFile::from_bundle(&dna_path).await.unwrap()
  }
  
#[tokio::test(flavor = "multi_thread")]
async fn can_call_must_get_agent_activity() {
    let dna = load_dna().await;

    // Set up conductors
    let mut conductors: SweetConductorBatch = SweetConductorBatch::from_config(2, ConductorConfig::default()).await;
    let alice_agentpubkey = SweetAgents::one(conductors[0].keystore()).await;
   
    let properties = SerializedBytes::try_from(DnaProperties {
        progenitor: alice_agentpubkey.clone().into_inner().to_vec(),
    }).unwrap();
    
    let dnas = &[dna.update_modifiers(DnaModifiersOpt { network_seed: None, properties: Some(properties), origin_time: None, quantum_time: None })];
    let app = conductors[0].setup_app_for_agent("demo", alice_agentpubkey.clone(), dnas).await.unwrap();
    let (alice,) = app.into_tuple();

    // Alice creates an SteamIdAttestation
    let _: Record = conductors[0]
        .call(&alice.zome("posts"), "create_post", Post {
            body: "my post".into(),
        })
        .await;
}
