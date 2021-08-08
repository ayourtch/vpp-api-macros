use vpp_macros::Message;
#[derive(Message, Debug)]
#[message_name_and_crc(Idiot_76fe)]
struct InterfaceAPIAddress{
    uid: u32, 
    name: String,
}

fn main(){
    println!("{}", InterfaceAPIAddress::get_message_name_and_crc());
    let builder = InterfaceAPIAddress::builder().uid(33).name("Faisal".to_owned()).build().unwrap();
    // let finalc = builder.build().unwrap();
    eprintln!("{:#?}",builder);
    assert_eq!(builder.uid,33);
    
}
