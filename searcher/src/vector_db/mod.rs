use actix::prelude::*;
use cxx::UniquePtr;
use usearch::ffi::{new_index, Index, IndexOptions, MetricKind, ScalarKind};

#[derive(Message)]
#[rtype(result = "Vec<(u64, f32)>")]
pub struct Query(pub Vec<f32>);

pub struct VectorDB {
    index: UniquePtr<Index>,
}

pub fn start() -> Addr<VectorDB> {
    let index = new_index(&IndexOptions {
        metric: MetricKind::Cos,
        quantization: ScalarKind::F16,
        dimensions: 384,
        connectivity: 0,
        expansion_add: 0,
        expansion_search: 0,
    })
    .unwrap();

    index
        .view("./kuberian.usearch")
        .expect("fail to load usearch index");

    VectorDB { index }.start()
}

impl Actor for VectorDB {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        info!("vector db service initiated");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        error!("vector db service shutdown")
    }
}

impl Handler<Query> for VectorDB {
    type Result = Vec<(u64, f32)>;

    fn handle(&mut self, msg: Query, _ctx: &mut Context<Self>) -> Self::Result {
        let result = self.index.search(&msg.0, 10).unwrap();
        let mut converted: Vec<(u64, f32)> = Vec::new();

        for (i, val) in result.keys.iter().enumerate() {
            converted.push((*val, *result.distances.get(i).unwrap()))
        }

        converted
    }
}
