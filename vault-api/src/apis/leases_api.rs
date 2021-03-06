/* 
 * Vault
 *
 * Hashicorp Vault API
 *
 * OpenAPI spec version: 0.7.2
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct LeasesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> LeasesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> LeasesApiClient<C> {
        LeasesApiClient {
            configuration: configuration,
        }
    }
}

pub trait LeasesApi {
    fn revoke_lease(&self, x_vault_token: &str, body: ::models::RevokeLeaseParameters) -> Box<Future<Item = (), Error = Error>>;
}


impl<C: hyper::client::Connect>LeasesApi for LeasesApiClient<C> {
    fn revoke_lease(&self, x_vault_token: &str, body: ::models::RevokeLeaseParameters) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let uri_str = format!("{}/sys/leases/revoke", configuration.base_path);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());

        {
            let mut headers = req.headers_mut();
            headers.set_raw("X-Vault-Token", x_vault_token);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|_| futures::future::ok(()))
        )
    }

}
