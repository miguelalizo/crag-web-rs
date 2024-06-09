#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Route {
    pub route: String,
}

impl Route {
    pub fn new(route: impl AsRef<str>) -> Self {
        Route {
            route: route.as_ref().to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_new() {
        let route = Route::new("/foo");
        assert_eq!(route.route, "/foo".to_owned());
    }
}
