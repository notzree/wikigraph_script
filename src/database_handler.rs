use crate::models::{LookupEntry, RedirectEntry};
use crate::schema::lookup::dsl::*;
use crate::schema::redirect::dsl::*;
use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};

pub trait DatabaseHandler {
    fn lookup_with_redirects(
        &mut self,
        input_title: &str,
    ) -> Result<LookupEntry, diesel::result::Error>;
    fn add_lookup_entry(&mut self, lookup_entry: &LookupEntry)
        -> Result<(), diesel::result::Error>;
    fn add_redirect_entry(
        &mut self,
        redirect_entry: &RedirectEntry,
    ) -> Result<(), diesel::result::Error>;
}

pub struct PostgresDatabaseHandler {
    connection: PgConnection,
}

impl PostgresDatabaseHandler {
    pub fn new(connection_string: &str) -> Result<Self, ConnectionError> {
        let connection = PgConnection::establish(connection_string)?;
        Ok(PostgresDatabaseHandler { connection })
    }
}

impl DatabaseHandler for PostgresDatabaseHandler {
    fn lookup_with_redirects(
        &mut self,
        input_title: &str,
    ) -> Result<LookupEntry, diesel::result::Error> {
        let result = redirect
            .filter(redirect_from.eq(input_title))
            .inner_join(lookup.on(redirect_to.eq(title)))
            .select(lookup::all_columns())
            .first::<LookupEntry>(&mut self.connection)
            .optional()?;

        match result {
            Some(entry) => {
                println!(" {:?}", entry);
                Ok(entry)
            }
            None => {
                println!("No redirect entry found for {}", input_title);
                // Fallback to directly querying the lookups table if no entry was found through redirects
                lookup
                    .filter(title.eq(input_title))
                    .first::<LookupEntry>(&mut self.connection)
            }
        }
    }
    fn add_lookup_entry(
        &mut self,
        lookup_entry: &LookupEntry,
    ) -> Result<(), diesel::result::Error> {
        match insert_into(lookup)
            .values(lookup_entry)
            .execute(&mut self.connection)
        {
            Ok(_) => Ok(()),
            Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                Ok(()) //keep going if we encounter a duplicate key error.
            }
            Err(e) => Err(e), //propogate any other errors
        }
    }
    fn add_redirect_entry(
        &mut self,
        redirect_entry: &RedirectEntry,
    ) -> Result<(), diesel::result::Error> {
        match insert_into(redirect)
            .values(redirect_entry)
            .execute(&mut self.connection)
        {
            Ok(_) => Ok(()),
            Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                Ok(()) //keep going if we encounter a duplicate key error.
            }
            Err(e) => Err(e), // For other errors, we will propgate
        }
    }
}
