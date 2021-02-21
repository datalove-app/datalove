// use crate::traits::NetworkAdmin;
// use async_trait::async_trait;
// use std::process::Command;

// #[derive(Debug)]
// pub struct Yggdrasil {
//     config: Config,
// }

// #[derive(Debug)]
// pub struct Config;

// const DAEMON_NAME: &str = "yggdrasil";
// const CTL_DAEMON_NAME: &str = "yggdrasil";

// #[async_trait]
// impl NetworkAdmin for Yggdrasil {
//     ///
//     async fn get_peers(&self) -> Result<(), ()> {
//         Command::new(CTL_DAEMON_NAME).arg("getPeers")
//     }

//     ///
//     async fn add_peer(&self) -> Result<(), ()> {
//         unimplemented!()
//     }

//     ///
//     async fn remove_peer(&self) -> Result<(), ()> {
//         unimplemented!()
//     }

//     ///
//     async fn get_dht(&self) -> Result<(), ()> {
//         unimplemented!()
//     }
// }
