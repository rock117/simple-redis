/// redis hello command
/// `HELLO [protover [AUTH username password] [SETNAME clientname]]`
pub(crate) struct Hello(Option<Opts>);

pub struct Opts {
    version: String,
    user: Option<User>,
    client_name: Option<String>,
}
impl Opts {
    pub fn new(
        version: String,
        user: Option<(String, String)>,
        client_name: Option<String>,
    ) -> Self {
        Self {
            version,
            user: user.map(|u| User {
                name: u.0,
                passwd: u.1,
            }),
            client_name,
        }
    }
}
struct User {
    name: String,
    passwd: String,
}
