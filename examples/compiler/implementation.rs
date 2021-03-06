use crate::class_table;
use crate::compiler::{CompilerDatabase, Interner};

/// Our "database" will be threaded through our application (though
/// 99% of the application only interacts with it through traits and
/// never knows its real name). It contains all the values for all of
/// our memoized queries and encapsulates **all mutable state that
/// persists longer than a single query execution.**
///
/// Databases can contain whatever you want them to, but salsa
/// requires you to add a `salsa::Runtime` member. Note
/// though: you should be very careful if adding shared, mutable state
/// to your context (e.g., a shared counter or some such thing). If
/// mutations to that shared state affect the results of your queries,
/// that's going to mess up the incremental results.
#[salsa::database(class_table::ClassTable)]
#[derive(Default)]
pub struct DatabaseImpl {
    runtime: salsa::Runtime<DatabaseImpl>,

    /// An interner is an example of shared mutable state that would
    /// be ok: although the interner allocates internally when you
    /// intern something new, this never affects any previously
    /// interned values, so it's not going to affect query results.
    interner: Interner,
}

/// This impl tells salsa where to find the salsa runtime.
impl salsa::Database for DatabaseImpl {
    fn salsa_runtime(&self) -> &salsa::Runtime<Self> {
        &self.runtime
    }

    fn salsa_runtime_mut(&mut self) -> &mut salsa::Runtime<Self> {
        &mut self.runtime
    }
}

/// In addition to the "query provider" traits, you may have other
/// trait requirements that your application needs -- you can
/// implement those yourself (in this case, an `interner`).
impl CompilerDatabase for DatabaseImpl {
    fn interner(&self) -> &Interner {
        &self.interner
    }
}
