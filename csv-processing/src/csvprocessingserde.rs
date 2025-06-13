use std::fs::File;
use csv::Reader;


#[derive(serde::Deserialize, Debug)]
struct Customer {
    // #[serde(rename = "Index")]
    // index: u64,

    #[serde(rename = "Customer Id")]
    customer_id: String,

    #[serde(rename = "First Name")]
    first_name: String,

    #[serde(rename = "Last Name")]
    last_name: String,

    // #[serde(rename = "Company")]
    // company: String,

    // #[serde(rename = "City")]
    // city: String,

    // #[serde(rename = "Country")]
    // country: String,

    // #[serde(rename = "Phone 1")]
    // phone_1: String,

    // #[serde(rename = "Phone 2")]
    // phone_2: String,

    // #[serde(rename = "Email")]
    // email: String,

    // #[serde(rename = "Subscription Date")]
    // subscription_date: String,

    // #[serde(rename = "Website")]
    // website: String
}

pub fn process_large_csv(filepath: &str) {

    let file = File::open(filepath).expect("Expected file exists");
    
    let mut rdr = Reader::from_reader(file);

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Customer = result.unwrap();
        println!("{:?} {:?} {:?}", record.customer_id, record.first_name, record.last_name);
    }
}
