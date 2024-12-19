use petgraph::prelude::NodeIndex;
pub fn get_nbr_indices<N>(node_idx: N, width: usize, height: usize) -> Vec<(u32, NodeIndex)>
where
    N: Into<NodeIndex>,
{
    let node_idx: NodeIndex = node_idx.into();
    let (x, y) = (
        (node_idx.index() % width) as i32,
        (node_idx.index() / width) as i32,
    );
    [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .iter()
        .enumerate()
        .filter_map(|(i, (x1, y1))| {
            let (x2, y2) = (x + x1, y + y1);
            // check bounds
            (x2 >= 0 && x2 < width as i32 && y2 >= 0 && y2 < height as i32)
                .then(|| (i, y2 as u32 * width as u32 + x2 as u32))
        })
        .map(|(cost, idx)| (cost as u32, NodeIndex::from(idx)))
        .collect()
}

pub fn get_x_y(idx: &NodeIndex, width: u32) -> (u32, u32) {
    (idx.index() as u32 % width, idx.index() as u32 / width)
}

pub fn get_node_idx<I>(x: I, y: I, width: I) -> NodeIndex
where
    I: Into<u32>,
{
    NodeIndex::from(y.into() * width.into() + x.into())
}
