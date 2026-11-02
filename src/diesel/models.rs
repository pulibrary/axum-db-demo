use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::diesel::schema::dogs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Dog {
    pub name: String,
}
