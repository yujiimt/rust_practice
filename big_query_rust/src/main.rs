use bigquery_storage::{Table, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Erro>>{
    // 1. load the desired secret (here, a service account key)
    let sa_key = yup_oauth2::read_service_account_key("dx-imt-0eceefac3745.json")
        .await?;
    // 2. create an authenticator
    let auth = yup_oauth2::ServiceAccountAuthenticator::builder(sa_key)
        .build()
        .await?;

    // 3. create a client
    let mut client = Client::new(auth).await?;

    //Reading the content of a table `bigquery-public-beta:london_bicycles.cycle_stations`
    let test_table = Table::new(
        "dx-imt",
        "tet",
        "test_tkd"
    );

    //create a new readsession; parent_projet_id is the ID of the GCP project
    // that owns the read job this does not download any data
    let mut read_session = client
        .read_session_builder(test_table)
        .parent_project_id("dx-imt".to_string())
        .build()
        .await?;
    
    let stream_reader = read_session
        .next_stream()
        .await?
        .expect("did not get any stream");
    
    let mut arrow_stream_reader = stream_reader
        .into_arrow_reader()
        .await?;
    
    let arrow_record_batch = arrow_stream_reader
        .next()
        .expect("no record batch")?;
    
    Ok(())
}