use tokio::time::{self,Duration};
#[tokio::main]
pub async fn decider(){
    let mut interval = time::interval(Duration::from_millis(100));
    interval.tick().await;
    
    loop{
        interval.tick().await;
        
    }
}
