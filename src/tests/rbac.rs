#[cfg(test)]
pub mod rbac_tests {
    use crate::authorization::{Permission, Role, Session};


    #[test]
    fn test_creation() {
        let r = Role::new("TestRole", false);
        assert!(!r.is_intrinsic());
        assert_eq!(r.get_name(), "TestRole");

        let p = Permission {

        };
        let s = Session::new(1000);
        assert!(!r.permission_applies(&p, &s))
    }

}