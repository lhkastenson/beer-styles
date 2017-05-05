extern crate rusted_cypher;

use std::env;
use rusted_cypher::GraphClient;

#[derive(Debug)]
pub struct Style {
    name: String,
    abv_low: f64,
    abv_high: f64,
    ibu_low: i32,
    ibu_high: i32,
    srm_low: i32,
    srm_high: i32,
    original_gravity_low: f64,
    original_gravity_high: f64,
    final_gravity_low: f64,
    final_gravity_high: f64
}

impl Style {
    pub fn new(name: String,
               abv_low: f64,
               abv_high: f64,
               ibu_low: i32,
               ibu_high: i32,
               srm_low: i32,
               srm_high: i32,
               original_gravity_low: f64,
               original_gravity_high: f64,
               final_gravity_low: f64,
               final_gravity_high: f64) -> Style {
        Style{name: name,
              abv_low: abv_low,
              abv_high: abv_high,
              ibu_low: ibu_low,
              ibu_high: ibu_high,
              srm_low: srm_low,
              srm_high: srm_high,
              original_gravity_low: original_gravity_low,
              original_gravity_high: original_gravity_high,
              final_gravity_low: final_gravity_low,
              final_gravity_high: final_gravity_high}
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_abv_low(&self) -> &f64 {
        &self.abv_low
    }

    fn get_abv_high(&self) -> &f64 {
        &self.abv_high
    }

    fn get_ibu_low(&self) -> &i32 {
        &self.ibu_low
    }

    fn get_ibu_high(&self) -> &i32 {
        &self.ibu_high
    }

    fn get_srm_low(&self) -> &i32 {
        &self.srm_low
    }

    fn get_srm_high(&self) -> &i32 {
        &self.srm_high
    }

    fn get_original_gravity_low(&self) -> &f64 {
        &self.original_gravity_low
    }

    fn get_original_gravity_high(&self) -> &f64 {
        &self.original_gravity_high
    }

    fn get_final_gravity_low(&self) -> &f64 {
        &self.final_gravity_low
    }

    fn get_final_gravity_high(&self) -> &f64 {
        &self.final_gravity_high
    }
}

impl PartialEq for Style {
    fn eq(&self, other: &Style) -> bool {
        self.get_name() == other.get_name() &&
            self.get_abv_low() == other.get_abv_low() &&
            self.get_abv_high() == other.get_abv_high() &&
            self.get_ibu_low() == other.get_ibu_low() &&
            self.get_ibu_high() == other.get_ibu_high() &&
            self.get_srm_low() == other.get_srm_low() &&
            self.get_srm_high() == other.get_srm_high() &&
            self.get_original_gravity_low() == other.get_original_gravity_low() &&
            self.get_original_gravity_high() == other.get_original_gravity_high() &&
            self.get_final_gravity_low() == other.get_final_gravity_low() &&
            self.get_final_gravity_high() == other.get_final_gravity_high()
    }
}
impl Eq for Style {}

#[allow(dead_code)]
pub fn get_beer_style(style_name: &String) -> Style {
    let neo4j_username = &env::var("NEO4JUSERNAME").unwrap();
    let neo4j_password = &env::var("NEO4JPASSWORD").unwrap();
    let neo4j_server_address = &env::var("NEO4JSERVERADDRESS").unwrap();
    let graph = GraphClient::connect(
        format!("http://{0}:{1}@{2}/db/data", neo4j_username, neo4j_password, neo4j_server_address)).unwrap();

    let result = graph.exec(
        format!("MATCH (s:Style {{name: '{}' }}) RETURN s.name, s.abvLow, s.abvHigh, s.ibuLow, s.ibuHigh, s.srmLow, s.srmHigh, s.originalGravityLow, s.originalGravityHigh, s.finalGravityLow, s.finalGravityHigh", &*style_name)).unwrap();
    let mut name = String::new();
    let mut abv_low: f64 = 0.0;
    let mut abv_high: f64 = 0.0;
    let mut ibu_low: i32 = 0;
    let mut ibu_high: i32 = 0;
    let mut srm_low: i32 = 0;
    let mut srm_high: i32 = 0;
    let mut original_gravity_low: f64 = 0.0;
    let mut original_gravity_high: f64 = 0.0;
    let mut final_gravity_low: f64 = 0.0;
    let mut final_gravity_high: f64 = 0.0;
    for row in result.rows() {
        name = row.get("s.name").unwrap();
        abv_low = row.get("s.abvLow").unwrap();
        abv_high = row.get("s.abvHigh").unwrap();
        ibu_low = row.get("s.ibuLow").unwrap();
        ibu_high = row.get("s.ibuHigh").unwrap();
        srm_low = row.get("s.srmLow").unwrap();
        srm_high = row.get("s.srmHigh").unwrap();
        original_gravity_low = row.get("s.originalGravityLow").unwrap();
        original_gravity_high = row.get("s.originalGravityHigh").unwrap();
        final_gravity_low = row.get("s.finalGravityLow").unwrap();
        final_gravity_high = row.get("s.finalGravityHigh").unwrap();
    }
    println!("{:?}", abv_low);
    let style = Style::new(name,
                           abv_low,
                           abv_high,
                           ibu_low,
                           ibu_high,
                           srm_low,
                           srm_high,
                           original_gravity_low,
                           original_gravity_high,
                           final_gravity_low,
                           final_gravity_high);
    println!("{:?}", style);
    style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let name = String::from("Belgian Dubbel");
        let abv_low = 6.0;
        let abv_high = 7.6;
        let ibu_low = 15;
        let ibu_high = 25;
        let srm_low = 10;
        let srm_high = 17;
        let original_gravity_low = 1.062;
        let original_gravity_high = 1.075;
        let final_gravity_low = 1.008;
        let final_gravity_high = 1.018;

        let actual = get_beer_style(&name);
        let expected = Style::new(name, abv_low, abv_high, ibu_low, ibu_high, srm_low, srm_high, original_gravity_low, original_gravity_high, final_gravity_low, final_gravity_high);
        
        assert_eq!(actual, expected);
    }
}















