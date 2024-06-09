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

impl From<&str> for Route {
    fn from(route: &str) -> Route {
        Route::new(route)
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

    #[test]
    fn test_from_str_for_route() {
        let route: Route = "/foo".into();
        assert_eq!(route, Route::new("/foo"));
    }
}
