/// The protocol to use for uptime monitoring requests
pub enum Protocol {
    HTTP_HTTPS,
    TCP,
    PING,
    DNS,
    PUSH,
    STEAM_GAME_SERVER,
}
