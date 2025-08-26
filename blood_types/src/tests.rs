use super::*;

#[test]
fn compatible_ab_neg_with_a_pos() {
    let blood_type: BloodType = "AB-".parse().unwrap();
    let other_bt: BloodType = "A+".parse().unwrap();
    assert!(!blood_type.can_receive_from(&other_bt));
}

#[test]
fn compatible_a_neg_with_a_pos() {
    let blood_type: BloodType = "A-".parse().unwrap();
    let other_bt: BloodType = "A+".parse().unwrap();
    assert!(!blood_type.can_receive_from(&other_bt));
}

#[test]
fn compatible_a_neg_with_ab_neg() {
    let blood_type: BloodType = "AB-".parse().unwrap();
    let other_bt: BloodType = "A-".parse().unwrap();
    assert!(blood_type.can_receive_from(&other_bt));
}

#[test]
fn compatible_ab_neg_with_o_pos() {
    let blood_type: BloodType = "AB-".parse().unwrap();
    let other_bt: BloodType = "O+".parse().unwrap();
    assert!(!blood_type.can_receive_from(&other_bt));
}

#[test]
fn compatible_ab_pos_with_o_pos() {
    let blood_type: BloodType = "AB+".parse().unwrap();
    let other_bt: BloodType = "O+".parse().unwrap();
    assert!(blood_type.can_receive_from(&other_bt));
}

#[test]
fn test_compatible_ab_neg_with_o_neg() {
    let blood_type: BloodType = "AB-".parse().unwrap();
    let other_bt: BloodType = "O-".parse().unwrap();
    assert!(blood_type.can_receive_from(&other_bt));
}

#[test]
fn test_antigen_ab_from_str() {
    let blood = "AB+";
    let blood_type: BloodType = blood.parse().unwrap();
    assert_eq!(blood_type.antigen, Antigen::AB);
    assert_eq!(blood_type.rh_factor, RhFactor::Positive);
}

#[test]
fn test_antigen_a_from_str() {
    let blood = "A-";
    let blood_type = blood.parse::<BloodType>().unwrap();
    assert_eq!(blood_type.antigen, Antigen::A);
    assert_eq!(blood_type.rh_factor, RhFactor::Negative);
}

#[test]
#[should_panic]
fn test_unexistent_blood_type() {
    let _blood_type: BloodType = "AO-".parse().unwrap();
}

#[test]
fn test_donors() {
    let mut givers = "AB+".parse::<BloodType>().unwrap().donors();
    println!("Before sorting {:?}", &givers);
    givers.sort();
    println!("{:?}", &givers);
    let mut expected = vec![
        "AB-".parse::<BloodType>().unwrap(),
        "A-".parse().unwrap(),
        "B-".parse().unwrap(),
        "O-".parse().unwrap(),
        "AB+".parse().unwrap(),
        "A+".parse().unwrap(),
        "B+".parse().unwrap(),
        "O+".parse().unwrap(),
    ];
    expected.sort();
    assert_eq!(givers, expected);
}

#[test]
fn test_a_neg_donors() {
    let mut givers = "A-".parse::<BloodType>().unwrap().donors();
    givers.sort();
    let mut expected: Vec<BloodType> = vec!["A-".parse().unwrap(), "O-".parse().unwrap()];
    expected.sort();
    assert_eq!(givers, expected);
}

#[test]
fn test_o_neg_donors() {
    let mut givers = "O-".parse::<BloodType>().unwrap().donors();
    givers.sort();
    let mut expected: Vec<BloodType> = vec!["O-".parse().unwrap()];
    expected.sort();
    assert_eq!(givers, expected);
}

#[test]
fn test_ab_pos_recipients() {
    let mut recipients: Vec<BloodType> = "AB+".parse::<BloodType>().unwrap().recipients();
    recipients.sort();
    let mut expected: Vec<BloodType> = vec!["AB+".parse().unwrap()];
    expected.sort();
    assert_eq!(recipients, expected);
}

#[test]
fn test_a_neg_recipients() {
    let mut recipients = "A-".parse::<BloodType>().unwrap().recipients();
    recipients.sort();
    let mut expected: Vec<BloodType> = vec![
        "A-".parse().unwrap(),
        "AB+".parse().unwrap(),
        "A+".parse().unwrap(),
        "AB-".parse().unwrap(),
    ];
    expected.sort();
    assert_eq!(recipients, expected);
}

#[test]
fn test_output() {
    let blood_type: BloodType = "O+".parse().unwrap();
    println!("recipients of O+ {:?}", blood_type.recipients());
    println!("donors of O+ {:?}", blood_type.donors());
    let another_blood_type: BloodType = "A-".parse().unwrap();
    println!(
        "donors of O+ can receive from {:?} {:?}",
        &another_blood_type,
        blood_type.can_receive_from(&another_blood_type)
    );
}