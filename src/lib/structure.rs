use crate::lib::{member::TrussMember, matrix::Matrix};

pub struct TrussStructure {
    members: Vec<TrussMember>,
    nodes: Vec<usize>,
}

impl TrussStructure {
    pub fn new(members: Vec<TrussMember>, nodes: Vec<usize>) -> TrussStructure {
        TrussStructure { members, nodes }
    }

    pub fn global_stiffness_matrix(&self) -> Matrix {
        let total_nodes = self.nodes.len();
        let mut gsm = Matrix::zeros(total_nodes * 2, total_nodes * 2);

        for member in &self.members {
            let mut member_matrix = member.expand(total_nodes);
            gsm = member_matrix.add(&gsm)
        }

        gsm
    }



}