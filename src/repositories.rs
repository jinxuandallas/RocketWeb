use diesel::{MysqlConnection, RunQueryDsl, ExpressionMethods};
use diesel::query_dsl::filter_dsl::FindDsl;
use crate::models::models::{NewPerson, Person};
use crate::schema::person;
use crate::schema::person::{age, name};

pub struct PersonRepository;

impl PersonRepository {
    pub fn find_all(c:&mut MysqlConnection) -> Vec<Person> {
        person::table
            // .limit(100)
            .load::<Person>(c)
            .expect("Error loading persons")
    }

    pub fn create(c:&mut MysqlConnection, person:&NewPerson) -> usize {
        diesel::insert_into(person::table)
            .values(person)
            .execute(c).expect("Error saving person")

    }

    pub fn delete(c:&mut MysqlConnection, person_id:i32)-> usize {
        diesel::delete(person::table.find(person_id)).execute(c).expect("Error deleting person")
    }

    pub fn save(c:&mut MysqlConnection, person: &Person) {
        diesel::update(person::table.find(&person.ID))
            .set((name.eq(&person.name),age.eq(&person.age)))
            .execute(c)
            .expect("Update failed");
    }
}