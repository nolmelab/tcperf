//! 러스트는 log로 트레이트 중심의 사용 인터페이스를 구현하고 
//! 실제 log 구현에서 처리를 담당하도록 한다. 이는 전형적인 러스트 방식이다. 
//! 
//!  
use log::{trace, debug, info, warn, error};
use log4rs;

pub fn run() {
    // 작업 폴더가 crate의 폴더이다. 
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    debug!("hello");
    info!("world!");
    warn!("hi!");
    trace!("hi trace!");
    error!("hi error!");

    // [1] log.yaml에 각 레벨별로 파일을 따로 지정한다. 좋은 방법이 아니므로 하나의 
    // 파일로 통합하여 지정할 수 있게 한다. 최소 레벨이므로 하나로 하면 된다.  
    // [2] 파일 이름을 패턴으로 남기기 
    // - 이건 yaml이 아닌 코드에서 설정해야 한다. 
    // [3] RollingFileAppender도 필요하다.  
    // - Policy가 분리되어 있으므로 가능할 수도 있겠다. 
    // - 코드에서 설정해야 하는 건 같다. 아직, 기능들이 조금씩 C#이나 C++ 보다 적은 
    //   경우가 많다.  

    // yaml로 설정하면 필요한 로그를 보면서 작업하는 건 어렵지 않다. 
}