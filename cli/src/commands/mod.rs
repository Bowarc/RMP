pub mod downloader;
pub mod player;

#[macro_export]
macro_rules! send_and_wait {
    ($client:expr, $command:expr, $response:ty, $match_arm:pat => $return_value:expr) => {{
        $client.send(ClientMessage::Command($command)).unwrap();

        let (_, message) = $client.recv(std::time::Duration::from_millis(100)).unwrap();
        match message {
            $match_arm => return $return_value,
            ServerMessage::Error(e) => {
                panic!("{e}")
            }
            _ => unreachable!(),
        }
    }};
}
