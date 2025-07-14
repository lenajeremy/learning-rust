trait PostState {
    fn change_state(&self) -> impl PostState;
}

pub struct Post {
    state: Box<dyn PostState>,
}

