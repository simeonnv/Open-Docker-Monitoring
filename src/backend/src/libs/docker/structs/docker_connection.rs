

#[derive(sqlx::FromRow, Debug)]
pub struct DockerConnection {
    pub name: String,
    pub host: String,
    pub protocol: String,
    pub cert_path: Option<String>
}