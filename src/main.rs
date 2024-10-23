use backend::inr_balance::InrBalance;
// use backend::run;

#[tokio::main]
async fn main() {
    let instance = InrBalance::instance();

    let mut instance = instance.lock().unwrap();

    instance.add_user("user1".to_string());

    instance.deduct_balance("user1".to_string(), 100);
    instance.deduct_locked("user1".to_string(), 50);
    instance.unlock_balance("user1".to_string(), 50);

    // second instance
    // let sec_inst = InrBalance::instance();
    //
    // let mut sec_inst = instance.lock().unwrap();
    //
    // sec_inst.deduct_balance("user1".to_string(), 100);
    // sec_inst.deduct_locked("user1".to_string(), 50);
    // sec_inst.unlock_balance("user1".to_string(), 50);
}
