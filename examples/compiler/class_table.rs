use crate::compiler;
use salsa::query_prototype;
use std::sync::Arc;

query_prototype! {
    pub trait ClassTableDatabase: compiler::CompilerDatabase {
        /// Get the fields.
        fn fields(class: DefId) -> Arc<Vec<DefId>> {
            type Fields;
        }

        /// Get the list of all classes
        fn all_classes(key: ()) -> Arc<Vec<DefId>> {
            type AllClasses;
        }

        /// Get the list of all fields
        fn all_fields(key: ()) -> Arc<Vec<DefId>> {
            type AllFields;
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct DefId(usize);

impl<DB: ClassTableDatabase> salsa::QueryFunction<DB> for AllClasses {
    fn execute(_: &DB, (): ()) -> Arc<Vec<DefId>> {
        Arc::new(vec![DefId(0), DefId(10)]) // dummy impl
    }
}

impl<DB: ClassTableDatabase> salsa::QueryFunction<DB> for Fields {
    fn execute(_: &DB, class: DefId) -> Arc<Vec<DefId>> {
        Arc::new(vec![DefId(class.0 + 1), DefId(class.0 + 2)]) // dummy impl
    }
}

impl<DB: ClassTableDatabase> salsa::QueryFunction<DB> for AllFields {
    fn execute(db: &DB, (): ()) -> Arc<Vec<DefId>> {
        Arc::new(
            db.all_classes(())
                .iter()
                .cloned()
                .flat_map(|def_id| {
                    let fields = db.fields(def_id);
                    (0..fields.len()).map(move |i| fields[i])
                }).collect(),
        )
    }
}