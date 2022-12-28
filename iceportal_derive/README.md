# iceportal_derive

Derives for iceportal ResponseObjects. The `ResponseObject` trait from the `iceportal` crate will be implemented on the struct with the default fetch function. Add the url argument like that: `#[response_object(url = "/api1/rs/tripInfo/connection/{eva_number}")]`. This will implemented in the ``T::url()`` function.