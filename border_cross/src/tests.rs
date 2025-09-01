use super::*;

#[allow(dead_code)]
fn setup_cars<'a>() -> Vec<&'a dyn Vehicle> {
    let cars: Vec<&dyn Vehicle> = vec![
        &Car {
            plate_nbr: "A3D5C7",
            model: "Model 3",
            horse_power: 325,
            year: 2010,
        },
        &Car {
            plate_nbr: "A785P7",
            model: "S",
            horse_power: 500,
            year: 1980,
        },
        &Car {
            plate_nbr: "D325C7",
            model: "300",
            horse_power: 300,
            year: 2000,
        },
        &Car {
            plate_nbr: "Z3KCH4",
            model: "Montana",
            horse_power: 325,
            year: 2020,
        },
    ];
    cars
}

#[allow(dead_code)]
fn setup_trucks<'a>() -> Vec<&'a dyn Vehicle> {
    let trucks: Vec<&dyn Vehicle> = vec![
        &Truck {
            plate_nbr: "V3D5CT",
            model: "Ranger",
            horse_power: 325,
            year: 2010,
            load_tons: 40,
        },
        &Truck {
            plate_nbr: "V785PT",
            model: "Silverado",
            horse_power: 500,
            year: 1980,
            load_tons: 40,
        },
        &Truck {
            plate_nbr: "DE2SC7",
            model: "Sierra",
            horse_power: 300,
            year: 2000,
            load_tons: 40,
        },
        &Truck {
            plate_nbr: "3DH432",
            model: "Cybertruck",
            horse_power: 325,
            year: 2020,
            load_tons: 40,
        },
    ];
    trucks
}

#[test]
fn all_car_models() {
    let cars = setup_cars();
    assert_eq!(all_models(cars), ["Model 3", "S", "300", "Montana"]);
}

#[test]
fn all_truck_models() {
    let trucks = setup_trucks();
    assert_eq!(
        all_models(trucks),
        ["Ranger", "Silverado", "Sierra", "Cybertruck"]
    );
}

#[test]
fn cars_and_trucks_models() {
    let mut vehicles = setup_cars();
    vehicles.extend(setup_trucks());
    assert_eq!(
        all_models(vehicles),
        [
            "Model 3",
            "S",
            "300",
            "Montana",
            "Ranger",
            "Silverado",
            "Sierra",
            "Cybertruck"
        ]
    );
}
