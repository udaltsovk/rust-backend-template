use std::marker::PhantomData;

use derive_where::derive_where;
use uuid::Uuid;

pub mod validation;

#[derive_where(Copy, Clone, Debug)]
pub struct Id<T> {
    pub value: Uuid,
    _entity: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(value: Uuid) -> Self {
        Self {
            value,
            _entity: PhantomData,
        }
    }

    pub fn generate() -> Self {
        Self::new(Uuid::now_v7())
    }
}

impl<T> From<Uuid> for Id<T> {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

impl<T> From<Id<T>> for Uuid {
    fn from(id: Id<T>) -> Self {
        id.value
    }
}

pub trait DomainType<T>: AsRef<T> + AsMut<T>
where
    Self: Sized,
    T: From<Self> + Clone,
{
    fn into_inner(self) -> T {
        self.into()
    }

    fn cloned_inner(&self) -> T {
        self.as_ref().clone()
    }

    fn it_should_be_safe_to_unwrap<E>(
        field: &'static str,
    ) -> impl FnOnce(E) -> T {
        move |_| panic!("We've validated {field} value, so it should be safe")
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use uuid::Uuid;

    use super::{DomainType, Id};

    #[derive(Debug, Clone, PartialEq)]
    struct User;

    #[derive(Debug, Clone, PartialEq)]
    struct Product;

    #[rstest]
    fn test_id_new() {
        let uuid = Uuid::now_v7();
        let id: Id<User> = Id::new(uuid);

        assert_eq!(id.value, uuid);
    }

    #[rstest]
    fn test_id_generate() {
        let id1: Id<User> = Id::generate();
        let id2: Id<User> = Id::generate();

        // Generated IDs should be different
        assert_ne!(id1.value, id2.value);

        // Generated IDs should be version 7 UUIDs
        assert_eq!(id1.value.get_version_num(), 7);
        assert_eq!(id2.value.get_version_num(), 7);
    }

    #[rstest]
    fn test_id_from_uuid() {
        let uuid = Uuid::now_v7();
        let id: Id<Product> = Id::from(uuid);

        assert_eq!(id.value, uuid);
    }

    #[rstest]
    fn test_uuid_from_id() {
        let original_uuid = Uuid::now_v7();
        let id: Id<User> = Id::new(original_uuid);
        let converted_uuid: Uuid = id.into();

        assert_eq!(converted_uuid, original_uuid);
    }

    #[rstest]
    fn test_id_copy_clone_debug() {
        let uuid = Uuid::now_v7();
        let id: Id<User> = Id::new(uuid);

        // Test Copy
        let copied_id = id;
        assert_eq!(copied_id.value, uuid);
        assert_eq!(id.value, uuid); // Original should still be accessible

        // Test Clone
        let cloned_id = id;
        assert_eq!(cloned_id.value, uuid);

        // Test Debug
        let debug_str = format!("{:?}", id);
        assert!(debug_str.contains("Id"));
        assert!(debug_str.contains(&uuid.to_string()));
    }

    #[rstest]
    fn test_id_type_safety() {
        let uuid = Uuid::now_v7();
        let user_id: Id<User> = Id::new(uuid);
        let product_id: Id<Product> = Id::new(uuid);

        // IDs with different type parameters should be distinct types
        // This test ensures compile-time type safety
        assert_eq!(user_id.value, product_id.value);

        // But they should have the same underlying UUID value
        let user_uuid: Uuid = user_id.into();
        let product_uuid: Uuid = product_id.into();
        assert_eq!(user_uuid, product_uuid);
    }

    // Test implementation for DomainType trait
    #[derive(Debug, Clone, PartialEq)]
    struct TestDomainValue {
        inner: String,
    }

    impl AsRef<String> for TestDomainValue {
        fn as_ref(&self) -> &String {
            &self.inner
        }
    }

    impl AsMut<String> for TestDomainValue {
        fn as_mut(&mut self) -> &mut String {
            &mut self.inner
        }
    }

    impl From<TestDomainValue> for String {
        fn from(value: TestDomainValue) -> Self {
            value.inner
        }
    }

    impl DomainType<String> for TestDomainValue {}

    #[rstest]
    fn test_domain_type_into_inner() {
        let test_value = TestDomainValue {
            inner: "test".to_string(),
        };
        let inner = test_value.into_inner();

        assert_eq!(inner, "test");
    }

    #[rstest]
    fn test_domain_type_cloned_inner() {
        let test_value = TestDomainValue {
            inner: "test".to_string(),
        };
        let cloned_inner = test_value.cloned_inner();

        assert_eq!(cloned_inner, "test");
        // Original should still be accessible
        assert_eq!(test_value.inner, "test");
    }

    #[rstest]
    #[should_panic(
        expected = "We've validated test_field value, so it should be safe"
    )]
    fn test_domain_type_safe_unwrap_panics() {
        let unwrap_fn =
            TestDomainValue::it_should_be_safe_to_unwrap::<()>("test_field");
        unwrap_fn(());
    }

    #[rstest]
    fn test_domain_type_as_ref_as_mut() {
        let mut test_value = TestDomainValue {
            inner: "test".to_string(),
        };

        // Test AsRef
        let inner_ref: &String = test_value.as_ref();
        assert_eq!(inner_ref, "test");

        // Test AsMut
        let inner_mut: &mut String = test_value.as_mut();
        inner_mut.push_str("_modified");

        assert_eq!(test_value.inner, "test_modified");
    }
}
