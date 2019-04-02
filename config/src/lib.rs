use std::env;

pub struct TestProxySettings {
    proxy: String,
    login: Option<String>,
    password: Option<String>,
}

pub struct MainConfig {
    proxy: Option<TestProxySettings>,
    // Остальных настроек пока нет
}

impl MainConfig {
    pub fn load() -> Self {

        let proxy = match env::var("TEST_PROXY") {
            Ok(val) => Some(TestProxySettings {
                proxy: val.to_string(),
                login: env::var("TEST_LOGIN").ok(),
                password: env::var("TEST_PASSWORD").ok(),
            }),
            Err(_e) => None,
        };
        MainConfig {
            proxy
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
