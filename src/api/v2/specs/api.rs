//! API v2 specifications trait.

use api::v2::models::*;

pub trait ApiSpecs {
	fn build(&self) -> ::Result<Build>;
	
	fn cats(&self) -> ::Result<Vec<i32>>;
	fn cat(&self, id: i32) -> ::Result<Cat>;
	
	fn colors(&self) -> ::Result<Vec<i32>>;
	fn color(&self, id: i32) -> ::Result<Color>;
	
	fn currencies(&self) -> ::Result<Vec<i32>>;
	fn currency(&self, id: i32) -> ::Result<Currency>;
	
	fn dungeons(&self) -> ::Result<Vec<String>>;
	fn dungeon(&self, id: &str) -> ::Result<Dungeon>;
	
	fn quaggans(&self) -> ::Result<Vec<String>>;
	fn quaggan(&self, id: &str) -> ::Result<Quaggan>;
}