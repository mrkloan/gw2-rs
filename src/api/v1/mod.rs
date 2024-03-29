//! GW2 REST API v1 implementation

mod models;
mod specs;
mod api;

pub mod prelude;

#[cfg(test)]
mod tests {
	use api::v1::prelude::*;
	
	#[test]
	fn build() {
		let api = Api::new(Api::builder().into());
		let build = api.build();
		
		assert!(build.is_ok());
		assert!(*build.unwrap() > 0);
	}
}