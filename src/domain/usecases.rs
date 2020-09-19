pub mod usecases {
    use crate::domain::entities::Keep;
    use crate::domain::repositories::repositories::KeepRepository;
    pub fn create_keep<R: KeepRepository>(keep: Keep, keep_repository: R) {
        keep_repository.save(keep);
    }

    pub fn get_all_keeps<R: KeepRepository>(keep_repository: R) -> Vec<Keep> {
        return keep_repository.keeps();
    }

    pub fn delete_keep<R: KeepRepository>(keep: Keep, keep_repository: R) {
        keep_repository.delete(keep);
    }

    #[cfg(test)]
    mod test {
        use crate::domain::entities::Keep;
        use crate::domain::repositories::repositories::MockKeepRepository;
        use crate::domain::usecases::usecases::{create_keep, delete_keep, get_all_keeps};
        use mockall::predicate;

        #[test]
        fn test_create_keep() {
            let keep = Keep {
                url: "url".to_string(),
            };

            let mut mock = MockKeepRepository::new();
            mock.expect_save()
                .with(predicate::eq(keep.to_owned()))
                .return_const(());

            create_keep(keep, mock);
        }

        #[test]
        fn test_get_all_keeps() {
            let keep = Keep {
                url: "url".to_string(),
            };
            let keep2 = Keep {
                url: "url2".to_string(),
            };
            let keeps = vec![keep, keep2];

            let mut mock = MockKeepRepository::new();
            mock.expect_keeps().return_const(keeps.clone());

            assert_eq!(keeps.clone(), get_all_keeps(mock));
        }

        #[test]
        fn test_delete_keep() {
            let keep = Keep {
                url: "url".to_string(),
            };
            let keep2 = Keep {
                url: "url2".to_string(),
            };
            let keeps = vec![keep2.clone()];

            let mut mock = MockKeepRepository::new();
            mock.expect_delete()
                .with(predicate::eq(keep2.clone()))
                .return_const(());

            delete_keep(keep2.clone(), mock);
        }
    }
}
