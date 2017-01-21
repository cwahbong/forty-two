use ::types::Uuid;

pub struct Buffer {
    pub id: Uuid,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer { id: Uuid::new_v4() }
    }

    // TODO actions such like edit
    //
}
