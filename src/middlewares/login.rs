use std::env;

use iron::{typemap, BeforeMiddleware, status};
use iron::error::IronError;
use iron::headers::{Authorization, Bearer};
use iron::prelude::*;

use jwt::{decode, Algorithm};
use dal::UserModels::User;
use slog::Logger;

use middlewares::MiddlewareErrorTypes;


pub struct LoginMiddleware {
	logger: Logger
}

impl LoginMiddleware {
	pub fn new(logger: &Logger) -> LoginMiddleware {
		let logger = logger.new(o!("module" => "LoginMiddleware"));
		LoginMiddleware {logger: logger}
	}
}

pub struct Value(User);

impl typemap::Key for LoginMiddleware { type Value = Value; }

impl BeforeMiddleware for LoginMiddleware {
	fn before(&self, req: &mut Request) -> IronResult<()> {

		let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

		let token = req.headers.get::<Authorization<Bearer>>();

		if let Some(&Authorization(ref bearer)) = token {
			match decode::<User>(&bearer.token, secret.as_ref(), Algorithm::HS512) {
				Ok(user) => {
					info!(self.logger, "Loggin succeed"; "user_id" => user.claims.id);
					req.extensions.insert::<LoginMiddleware>(Value(user.claims));
					Ok(())
				},
				Err(error) => {
					info!(self.logger, "Loggin failed"; "reason" => "JWT error", "details" => format!("{:?}", error));
					Err(IronError::new(error, status::Unauthorized))
				}
			}
		}
		else {
			info!(self.logger, "Loggin failed"; "reason" => "no bearer token found", "details" => format!("{:?}", token));
			Err(IronError::new(MiddlewareErrorTypes::AuthorizationError, status::Unauthorized))
		}
	}
}

pub trait LoginReqExt {
	fn get_user_data(&self) -> &User;
}

impl <'a, 'b>LoginReqExt for Request <'a, 'b> {
	fn get_user_data(&self) -> &User {
		let &Value(ref user) = self.extensions.get::<LoginMiddleware>().expect("LoginMiddleware not in the chain");
		user
	}
}
