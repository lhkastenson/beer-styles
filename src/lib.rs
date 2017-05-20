extern crate rusted_cypher;

use std::env;
use rusted_cypher::GraphClient;
use rusted_cypher::error::GraphError;
use rusted_cypher::cypher::result::CypherResult;

#[derive(Debug)]
pub struct Style {
    name: String,
    abv_low: f64,
    abv_high: f64,
    ibu_low: i32,
    ibu_high: i32,

    srm_low: f64,
    srm_high: f64,
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
               srm_low: f64,
               srm_high: f64,
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

    fn get_srm_low(&self) -> &f64 {
        &self.srm_low
    }

    fn get_srm_high(&self) -> &f64 {
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


pub fn get_graph_connection() -> Result<GraphClient, GraphError> {
    let neo4j_username = &env::var("NEO4JUSERNAME").unwrap();
    let neo4j_password = &env::var("NEO4JPASSWORD").unwrap();
    let neo4j_server_address = &env::var("NEO4JSERVERADDRESS").unwrap();
    GraphClient::connect(
        format!("http://{0}:{1}@{2}/db/data", neo4j_username, neo4j_password, neo4j_server_address))
}

pub fn create_beer_style(style: &Style) -> Result<String, GraphError> {
    let graph = try!(get_graph_connection());
    try!(graph.exec(
        format!("CREATE (s:Style {{name: '{0}', abvLow: {1}, abvHigh: {2}, ibuLow: {3}, ibuHigh: {4}, srmLow: {5}, srmHigh: {6}, originalGravityLow: {7}, originalGravityHigh: {8}, finalGravityLow: {9}, finalGravityHigh: {10} }}) RETURN s",
                style.name,
                style.abv_low,
                style.abv_high,
                style.ibu_low,
                style.ibu_high,
                style.srm_low,
                style.srm_high,
                style.original_gravity_low,
                style.original_gravity_high,
                style.final_gravity_low,
                style.final_gravity_high)));

    Ok(style.name.clone())
}

pub fn get_beer_style(style_name: &String) -> Result<Style, GraphError> {
    let graph = try!(get_graph_connection());
    let result = try!(graph.exec(
        format!("MATCH (s:Style {{name: '{}' }}) RETURN s.name, s.abvLow, s.abvHigh, s.ibuLow, s.ibuHigh, s.srmLow, s.srmHigh, s.originalGravityLow, s.originalGravityHigh, s.finalGravityLow, s.finalGravityHigh",
                &*style_name)));
    Ok(build_style_from_result(result))
}

pub fn update_beer_style(style: &Style) -> Result<Style, GraphError> {
    let graph = try!(get_graph_connection());
    let result = try!(graph.exec(
        format!("MERGE (s:Style {{name: '{0}' }}) ON MATCH SET s.abvLow = {1}, s.abvHigh = {2}, s.ibuLow = {3}, s.ibuHigh = {4}, s.srmLow = {5}, s.srmHigh = {6}, s.originalGravityLow = {7}, s.originalGravityHigh = {8}, s.finalGravityLow = {9}, s.finalGravityHigh = {10} RETURN s.name, s.abvLow, s.abvHigh, s.ibuLow, s.ibuHigh, s.srmLow, s.srmHigh, s.originalGravityLow, s.originalGravityHigh, s.finalGravityLow, s.finalGravityHigh",
                style.name,
                style.abv_low,
                style.abv_high,
                style.ibu_low,
                style.ibu_high,
                style.srm_low,
                style.srm_high,
                style.original_gravity_low,
                style.original_gravity_high,
                style.final_gravity_low,
                style.final_gravity_high)));

    Ok(build_style_from_result(result))
}

pub fn delete_beer_style(name: &String) -> Result<bool, GraphError> {
    let graph = try!(get_graph_connection());
    let result = try!(graph.exec(
        format!("MATCH (s:Style {{name: '{}' }}) DELETE s RETURN count(s)", name)));
    let mut query_result: bool = false;
    for row in result.rows() {
        let count: i32 = row.get("count(s)").unwrap();
        query_result = count > 0;
    }

    Ok(query_result)

}

fn build_style_from_result(result: CypherResult) -> Style {
    let mut name = String::new();
    let mut abv_low: f64 = 0.0;
    let mut abv_high: f64 = 0.0;
    let mut ibu_low: i32 = 0;
    let mut ibu_high: i32 = 0;
    let mut srm_low: f64 = 0.0;
    let mut srm_high: f64 = 0.0;
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

    Style::new(name,
               abv_low,
               abv_high,
               ibu_low,
               ibu_high,
               srm_low,
               srm_high,
               original_gravity_low,
               original_gravity_high,
               final_gravity_low,
               final_gravity_high)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_style_successful_test() {
        let name = String::from("Test Style For Get");
        let abv_low = 6.0;
        let abv_high = 7.6;
        let ibu_low = 15;
        let ibu_high = 25;

        let srm_low = 10.0;
        let srm_high = 17.0;
        let original_gravity_low = 1.062;
        let original_gravity_high = 1.075;
        let final_gravity_low = 1.008;
        let final_gravity_high = 1.018;

        let style = Style::new(name, abv_low, abv_high, ibu_low,
                               ibu_high, srm_low, srm_high, original_gravity_low,
                               original_gravity_high, final_gravity_low, final_gravity_high);

        let _ = create_beer_style(&style);

        match get_beer_style(&style.name) {
            Ok(actual) => {
                let expected = style;

                let _ = delete_beer_style(&actual.name);

                assert_eq!(actual, expected)
            }
            Err(err) => panic!("Error: something bad happened with get: {0}", err)
        }

    }

    #[test]
    fn create_style_successful_test() {
        let name = String::from("Test Style For Create");
        let abv_low = 4.4;
        let abv_high = 5.8;
        let ibu_low = 20;
        let ibu_high = 35;
        let srm_low = 10.0;
        let srm_high = 16.0;
        let original_gravity_low = 1.044;
        let original_gravity_high = 1.060;
        let final_gravity_low = 1.013;
        let final_gravity_high = 1.017;

        let style = Style::new(name, abv_low, abv_high, ibu_low, ibu_high, srm_low, srm_high, original_gravity_low, original_gravity_high, final_gravity_low, final_gravity_high);

        match create_beer_style(&style) {
            Ok(actual) => {
                let expected = style.name;

                let _ = delete_beer_style(&actual);

                assert_eq!(actual, expected);
            }
            Err(err) => panic!("Error: something bad happened with create: {0}", err)
        }
    }

    #[test]
    fn update_style_successful_test() {

        let name = String::from("Test Style For Update");
        let abv_low = 0.0;
        let abv_high = 0.0;
        let ibu_low = 0;
        let ibu_high = 0;
        let srm_low = 0.0;
        let srm_high = 0.0;
        let original_gravity_low = 0.0;
        let original_gravity_high = 0.0;
        let final_gravity_low = 0.0;
        let final_gravity_high = 0.0;

        let original_style = Style::new(name, abv_low, abv_high, ibu_low, ibu_high, srm_low, srm_high, original_gravity_low, original_gravity_high, final_gravity_low, final_gravity_high);

        let _ = create_beer_style(&original_style);


        let name = String::from("Test Style For Update");
        let abv_low = 4.2;
        let abv_high = 5.8;
        let ibu_low = 30;
        let ibu_high = 45;
        let srm_low = 3.5;
        let srm_high = 6.0;
        let original_gravity_low = 1.044;
        let original_gravity_high = 1.060;
        let final_gravity_low = 1.013;
        let final_gravity_high = 1.017;
        let edited_style = Style::new(name, abv_low, abv_high, ibu_low, ibu_high, srm_low, srm_high, original_gravity_low, original_gravity_high, final_gravity_low, final_gravity_high);

        match update_beer_style(&edited_style) {
            Ok(updated_style) => {
                let _ = delete_beer_style(&updated_style.name);
                assert_ne!(updated_style, original_style);
            }
            Err(err) => panic!("Error: something bad happened with update: {0}", err)
        }
    }

    #[test]
    fn delete_style_not_found_successful_test() {
        let name = String::from("Test Style That Does Not Exist!");
        match delete_beer_style(&name) {
            Ok(actual) => {
                let expected = false;
                assert_eq!(actual, expected)
            }
            Err(err) => panic!("Error: something bad happened with delete: {0}", err)
        }
    }

    #[test]
    fn delete_style_successful_test() {
        let name = String::from("Test Style That Does Exist");
        let abv_low = 4.2;
        let abv_high = 5.8;
        let ibu_low = 30;
        let ibu_high = 45;
        let srm_low = 3.5;
        let srm_high = 6.0;
        let original_gravity_low = 1.044;
        let original_gravity_high = 1.060;
        let final_gravity_low = 1.013;
        let final_gravity_high = 1.017;
        let style = Style::new(name, abv_low, abv_high, ibu_low, ibu_high, srm_low, srm_high, original_gravity_low, original_gravity_high, final_gravity_low, final_gravity_high);

        let _ = create_beer_style(&style);

        match delete_beer_style(&style.name) {
            Ok(actual) => {
                let expected = true;
                assert_eq!(actual, expected)
            }
            Err(err) => panic!("Error: something bad happened with delete: {0}", err)
        }
    }
}
