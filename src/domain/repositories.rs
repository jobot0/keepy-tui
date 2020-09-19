pub mod repositories {
    #[cfg(test)]
    use mockall::automock;

    use crate::domain::entities::Keep;

    #[cfg_attr(test, automock)]
    pub trait KeepRepository {
        fn save(&self, keep: Keep) -> ();
        fn keeps(&self) -> Vec<Keep>;
        fn delete(&self, keep: Keep) -> ();
    }
}
