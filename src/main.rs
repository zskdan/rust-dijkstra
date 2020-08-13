type Vertex = char;

#[derive(Debug)]
struct Connection {
    peers   : (Vertex, Vertex),
    weight  : u32,
}

#[derive(Debug)]
struct Graph {
    connections : Vec<Connection>,
    vertices    : Vec<Vertex>,
}

#[derive(Debug)]
struct Road {
    vertex     : Vertex,
    distance   : u32,
    via_vertex : Vertex,
}

#[derive(Debug)]
struct DijkstraTable {
    start_vertex : Vertex,
    roads        : Vec<Road>,
    unvisited    : Vec<Vertex>,
}

impl DijkstraTable {
    fn get_distance(&self, vertex: Vertex) -> u32 {
        /* Work on the ref of ref in order to borrow an immutable ref from
           a mutable object */
        (&self).roads.iter()
                  .find(|road| road.vertex == vertex)
                  .map(|road| road.distance)
                  .unwrap_or(0)
    }

    fn get_road_mut(&mut self, vertex: &Vertex) -> Option<&mut Road> {
        self.roads.iter_mut()
                  .find(|road| road.vertex == *vertex)
    }

    fn get_road(&self, vertex: &Vertex) -> Option<&Road> {
        self.roads.iter()
                  .find(|road| road.vertex == *vertex)
    }

    fn get_next_unvisited(&self) -> Option<Vertex> {
        let mut min = u32::MAX;
        let mut next = None;

        for v in &self.unvisited {
            match self.get_road(&v) {
                None => break,
                Some(r) => {
                    if r.distance < min {
                        min = r.distance;
                        next = Some(*v);
                    }
                }
            }
        }
        next
    }

    fn remove(&mut self, v : &Vertex) {
        match self.unvisited.iter().position(|vertex| vertex==v) {
            None => (),
            Some(index) => {
                self.unvisited.remove(index);

    fn new(graph: &Graph, start: Vertex) -> DijkstraTable {
        let mut table = DijkstraTable {
            start_vertex : start,
            roads        : Vec::new(),
            unvisited    : graph.vertices.clone(),
        };

        for v in &graph.vertices {
            let mut road = Road::new(*v);

            if *v == start {
                road.distance = 0;
            }

            table.roads.push(road);
        }

        loop {
            match table.get_next_unvisited() {
                None => break,
                Some(v) => {
                    //println!("{}##################",v);
                    for n in graph.get_neighbours(&v) {
                        let d = graph.get_weight((v, *n));
                        let k = d + table.get_distance(v);
                        let rn = table.get_road_mut(n);
                        if let Some(rn) = rn {
                            if k < rn.distance {
                                rn.via_vertex = v;
                                rn.distance = k;
                            }
                        }
                    }
                    table.remove(v);
                    //println!(" {:#?} ", table);
                }
            }
        }
        table
    }
}

impl Road {
    fn new(from: Vertex) -> Road {
        Road {
            vertex      : from,
            distance    : u32::MAX,
            via_vertex  : '-',
        }
    }
}

impl Graph {
    fn get_weight(&self, (a, b): (Vertex, Vertex)) -> u32 {
        self.connections.iter()
                        .find(|c| c.peers == (a,b) || c.peers == (b,a))
                        .map(|c| c.weight)
                        .unwrap_or(0)
    }

    fn get_neighbours(&self, vertex: &Vertex) -> Vec<&Vertex> {
        let mut neighbours : Vec<&Vertex> = Vec::new();

        for c in &self.connections {
            if c.peers.0 == *vertex {
                neighbours.push(&c.peers.1);
            } else if c.peers.1 == *vertex {
                neighbours.push(&c.peers.0);
            }
        }

        neighbours
    }

    fn vertices_from_connections(conns : &Vec<Connection>) -> Vec<Vertex> {
        let mut verts : Vec<Vertex> = Vec::new();

        for c in conns {
            if ! verts.contains(&c.peers.0) {
                verts.push(c.peers.0);
            }
            if ! verts.contains(&c.peers.1) {
                verts.push(c.peers.1);
            }
        }
        verts
    }

    fn new(conns: Vec<Connection>) -> Graph {
        Graph {
            vertices    : Graph::vertices_from_connections(&conns),
            connections : conns,
        }
    }

}

fn main() {
    let graph = Graph::new(
        vec![
            Connection {
                peers: ('A', 'B'),
                weight: 6,
            },
            Connection {
                peers: ('A', 'D'),
                weight: 1,
            },
            Connection {
                peers: ('D', 'E'),
                weight: 1,
            },
            Connection {
                peers: ('D', 'B'),
                weight: 2,
            },
            Connection {
                peers: ('E', 'B'),
                weight: 2,
            },
            Connection {
                peers: ('E', 'C'),
                weight: 5,
            },
            Connection {
                peers: ('B', 'C'),
                weight: 5,
            },
        ]
    );

    let dt = DijkstraTable::new(&graph, 'A');
    println!(" Dijkstra of 'A': {:#?}", dt);
}
