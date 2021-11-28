fn main() {
    let pound = 1.00;

    //convert our pound to kilos
     let kilo = pounds_to_kilo(pound);
     println!("resulting kilos: {:?}kg", kilo);

}

fn pounds_to_kilo(pounds: f64)-> f64{
    //m(kg) = m(lb) × 0.45359237
    pounds*0.45359237
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pounds_to_kilo_test(){
        assert_eq!(0.45359237,pounds_to_kilo(1.00));
        assert_eq!(0.90718474,pounds_to_kilo(2.00));
    }
}
