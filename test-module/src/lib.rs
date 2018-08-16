#[macro_use]
extern crate pamsm;
extern crate time;

use pamsm::{PamServiceModule, Pam, PamFlag, PamError};

struct PamTime;

impl PamServiceModule for PamTime {
    fn authenticate(self: &Self, pamh: Pam, _: PamFlag, args: Vec<String>) -> PamError {

        // If you need login/password here, that works like this:
        //
        //  let user = match pamh.get_user(None) {
        //      Ok(Some(u)) => u,
        //      Ok(None) => return PamError::USER_UNKNOWN,
        //      Err(e) => return e,
        //  };
		//
        //  let pass = match pamh.get_authtok(None) {
        //      Ok(Some(p)) => p,
        //      Ok(None) => return PamError::AUTH_ERR,
        //      Err(e) => return e,
        //  };

        // Only allow authentication when it's 4 AM
        let hour = time::now().tm_hour;
        if hour != 4 {
            PamError::SUCCESS
        } else {
            PamError::AUTH_ERR
        }
    }
}

pamsm_init!(Box::new(PamTime));
