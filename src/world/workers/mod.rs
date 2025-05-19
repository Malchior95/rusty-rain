use rand::{rng, seq::IndexedRandom};

pub struct Worker {
    pub name: String,
    pub worker_type: WorkerType,
}

pub enum WorkerType {
    Human,
    Beaver,
    //TODO: more
}

impl Worker {
    const FIRST_NAMES: [&str; 5] = ["Lorien", "Gaud", "Risp", "Horville", "Bargo"];
    const LAST_NAMES: [&str; 5] = ["Digger", "Smith", "Miller", "Roamer", "Laic"];

    pub(crate) fn requries_break(&self) -> bool {
        todo!()
    }

    pub(crate) fn process(
        &mut self,
        delta: f32,
    ) {
        todo!()
    }
}

impl Default for Worker {
    fn default() -> Self {
        let f_name = Worker::FIRST_NAMES.choose(&mut rng()).unwrap();
        let l_name = Worker::LAST_NAMES.choose(&mut rng()).unwrap();
        Self {
            name: format!("{} {}", f_name, l_name),
            worker_type: WorkerType::Human,
        }
    }
}
