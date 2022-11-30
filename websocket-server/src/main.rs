use ws::*;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Client {
    out: Sender,
    user_id: u32,
    client_list: Rc<RefCell<Vec<Client>>>,
}

impl Handler for Client {
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        self.client_list.borrow_mut().iter().filter(|user| user.user_id != self.user_id)
        .for_each(|user|{
             user.out.send(msg.clone());
        });
        Ok(())
    }

    fn on_open<'a>(&'a mut self, _: Handshake) -> ws::Result<()> {
      println!("current user is below.");
      self.client_list.borrow_mut().iter().for_each(|client|{
          println!("{:?}", client.user_id);
      });
      Ok(())
    }
}

fn main() {
  let client_list = Rc::new(RefCell::new(vec!()));

  listen("127.0.0.1:8080", move |out| { 
    let client = Client { 
      out: out.clone(),
      user_id: rand::thread_rng().gen(),
      client_list: client_list.clone(),
    };
    client_list.borrow_mut().push(client.clone());

    client
  }).unwrap()
}