mod consume;
mod produce;
mod message;

pub use runner::*;

use super::TestDriver;

mod runner {

    use std::time::Duration;
    use flv_future_aio::timer::sleep;

    use async_trait::async_trait;

    use crate::TestOption;
    use super::*;

    /// simple smoke test runner which tests
    pub struct SmokeTestRunner {
        option: TestOption,
    }

    impl SmokeTestRunner {
        pub fn new(option: TestOption) -> Self {
            Self { option }
        }

        async fn produce_and_consume_cli(&self) {
            if self.option.produce() {
                super::produce::produce_message(&self.option).await;
            } else {
                println!("produce skipped");
            }

            sleep(Duration::from_secs(1)).await;

            if self.option.test_consumer() {
                super::consume::validate_consume_message(&self.option).await;
            } else {
                println!("consume test skipped");
            }
        }
    }

    #[async_trait]
    impl TestDriver for SmokeTestRunner {
        /// run tester
        async fn run(&self) {
            //use futures::future::join_all;
            //use futures::future::join;

            //use flv_future_aio::task::run_block_on;

            //let mut listen_consumer_test = vec ![];

            println!("start testing...");

            /*
            if self.0.test_consumer() {
                for i in 0..self.0.replication() {
                    listen_consumer_test.push(consume::validate_consumer_listener(i,&self.0));
                }
            }
            */

            /*
            run_block_on(
                join(
                    self.produce_and_consume_cli(&target),
                    join_all(listen_consumer_test)
                ));
            */
            self.produce_and_consume_cli().await;
        }
    }
}

mod client {

    use flv_client::profile::*;
    use flv_client::client::*;

    use crate::TestOption;
    use crate::tls::Cert;

    #[allow(unused)]
    pub async fn get_client(option: &TestOption) -> ScClient {
        let tls_option = if option.tls() {
            let client_cert = Cert::load_client();

            Some(TlsConfig::File(TlsClientConfig {
                client_cert: client_cert.cert.display().to_string(),
                client_key: client_cert.key.display().to_string(),
                ca_cert: client_cert.ca.display().to_string(),
                domain: "fluvio.local".to_owned(),
            }))
        } else {
            None
        };

        let config = ScConfig::new(None, tls_option).expect("connect");
        config.connect().await.expect("should connect")
    }
}
