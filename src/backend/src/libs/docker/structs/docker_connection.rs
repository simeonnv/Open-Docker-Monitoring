// use bollard::ClientVersion;



#[derive(sqlx::FromRow, Debug)]
pub struct DockerConnection {
    pub name: String,
    pub host: String,
    pub protocol: String,
    // pub version: ClientVersion
}