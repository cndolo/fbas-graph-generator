use fbas_analyzer::{Fbas, NodeId};
use fbas_reward_distributor::*;

/// Optionally rank nodes using S-S Power Index or NodeRank and return a sorted list of nodes
pub(crate) fn compute_influence(
    node_ids: &[NodeId],
    fbas: &Fbas,
    alg: Option<RankingAlg>,
    use_pks: bool,
    qi_check: bool,
) -> Vec<NodeRanking> {
    let rankings = if let Some(rank_algo) = alg {
        rank_nodes(fbas, rank_algo, qi_check)
    } else {
        // all nodes have the same weight
        fbas.all_nodes().iter().map(|_| 1.0).collect()
    };
    create_node_ranking_report(node_ids, rankings, fbas, use_pks)
}

/// Gets an FBAS and returns a list or neighbours for every node
pub(crate) fn generate_adjacency_list(fbas: &Fbas) -> Vec<String> {
    let mut adj_list: Vec<Vec<String>> = Vec::default();
    for node in fbas.all_nodes().into_iter() {
        let own_list = vec![node.to_string()];
        adj_list.push(own_list);
    }
    for node in fbas.all_nodes().into_iter() {
        // this node has an edge to these nodes
        let contained_nodes = fbas.get_quorum_set(node).unwrap().contained_nodes();
        for target in contained_nodes.into_iter() {
            adj_list[target].push(node.to_string());
        }
    }

    adj_list.iter().map(|nodelist| nodelist.join(" ")).collect()
}

/// Gets a vec of tupels of the type (NodeId, PublicKey, Score) and returns them as strings in the
/// same order, i.e. Vec<"Id, PK, score">
pub fn generate_node_list_with_weight(rankings: &[NodeRanking]) -> Vec<String> {
    let mut nodelist: Vec<String> = Vec::default();
    for node in rankings {
        let line = format!("{},{},{}\n", node.0, node.1, node.2);
        nodelist.push(line)
    }
    nodelist
}

#[cfg(test)]
mod tests {
    use super::*;
    use fbas_analyzer::Fbas;
    use std::path::Path;

    #[test]
    fn output_adjacency_list_is_ok() {
        let fbas = Fbas::from_json_file(Path::new("test_data/trivial.json"));
        let actual = generate_adjacency_list(&fbas);
        let expected = vec!["0 0 1 2", "1 0 1 2", "2 0 1 2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn output_nodelist_is_ok() {
        let rankings = vec![
            (0, "nodeA".to_string(), 0.0),
            (1, "nodeB".to_string(), 0.1),
            (2, "nodeC".to_string(), 0.2),
        ];
        let actual = generate_node_list_with_weight(&rankings);
        let expected = vec!["0,nodeA,0\n", "1,nodeB,0.1\n", "2,nodeC,0.2\n"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn compute_unweighted_node_rankings() {
        let fbas = Fbas::from_json_file(Path::new("test_data/trivial.json"));
        let node_ids: Vec<NodeId> = (0..fbas.all_nodes().len()).collect();
        let qi_check = false;
        let use_pks = false;
        let alg = None;
        let actual = compute_influence(&node_ids, &fbas, alg, use_pks, qi_check);
        let expected = vec![
            (0, String::from(""), 1.0),
            (1, String::from(""), 1.0),
            (2, String::from(""), 1.0),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn compute_weighted_node_rankings() {
        let fbas = Fbas::from_json_file(Path::new("test_data/trivial.json"));
        let node_ids: Vec<NodeId> = (0..fbas.all_nodes().len()).collect();
        let qi_check = false;
        let use_pks = false;
        let alg = Some(RankingAlg::PowerIndexEnum(None));
        let actual = compute_influence(&node_ids, &fbas, alg, use_pks, qi_check);
        let expected = vec![
            (0, String::from(""), 0.333),
            (1, String::from(""), 0.333),
            (2, String::from(""), 0.333),
        ];
        assert_eq!(actual, expected);
    }
}
