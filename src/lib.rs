use zed_extension_api::{Extension, register_extension};

struct JustOneSpace {
    // ... state
}

impl Extension for JustOneSpace {
	fn new() -> Self
		where Self: Sized
	{
		todo!()
	}
}

register_extension!(JustOneSpace);
