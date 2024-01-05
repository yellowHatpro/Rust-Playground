
#[derive(Debug,Clone)]
struct TLSCert {
    key: String,
    cert: String,
}
type ms = u32;

#[derive(Debug)]
struct Server {
    host: String,
    port: u16,
    tls: Option<TLSCert>,
    hot_reload: bool,
    timeout: ms,
}

pub struct ServerBuilder {
    host: String,
    port: u16,
    tls: Option<TLSCert>,
    hot_reload: Option<bool>,
    timeout: Option<ms>,
}

impl ServerBuilder {
    fn tls(&mut self, tls: TLSCert) -> &mut Self {
        self.tls = Some(tls);
        self
    }

    fn hot_reload(&mut self, hot_reload: bool) -> &mut Self {
        self.hot_reload = Some(hot_reload);
        self
    }

    fn timeout(&mut self, timeout: ms) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }
    fn build(&mut self) -> Server {
        Server {
            host: self.host.clone(),
            port: self.port,
            tls: self.tls.clone(),
            hot_reload: self.hot_reload.unwrap_or_default(),
            timeout: self.timeout.unwrap_or(2000)
        }
    }
}

impl Server {
    fn new(host: String, port: u16) -> ServerBuilder {
        ServerBuilder {
            host,
            port,
            tls: None,
            hot_reload: None,
            timeout: None,
        }
    }
}

fn main() {
    let host: String = "localhost".to_owned();
    let port: u16 = 8080;
    let cert: TLSCert = TLSCert { key: "...".to_owned(), cert: "...".to_owned() };
    let basic_server = Server::new(host.clone(), port).build();
    let tls_server = Server::new(host.clone(),port).tls(cert.clone()).build();
    let server  = Server::new(host.clone(),port)
        .tls(cert.clone())
        .hot_reload(true)
        .timeout(5000)
        .build();
}