pub struct PlayerConfig {
    song_path: std::path::PathBuf,
}

impl PlayerConfig {
    pub fn song_path(&self) -> &std::path::PathBuf {
        &self.song_path
    }
}

impl Default for PlayerConfig {
    fn default() -> Self {
        use std::path::PathBuf;
        PlayerConfig {
            song_path: PathBuf::from("./songs/"),
        }
    }
}

pub fn default_proxy_config() -> networking::proxy::ProxyConfig {
    networking::proxy::ProxyConfig {
        addr: shared::DEFAULT_ADDRESS,
        run_tps: 20,
        stat_cfg: networking::stats::StatConfig {
            bps: networking::stats::config::BpsConfig { enabled: true },
            rtt: networking::stats::config::RttConfig {
                enabled: true,
                ping_request_delay: std::time::Duration::from_secs(5),
            },
        },
        keep_msg_while_disconnected: false,
        auto_reconnect: false, // This doens't makes sense for a server lol,
    }
}
