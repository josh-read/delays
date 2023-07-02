def graph_search(adjacency_list, start_node, finish_node):
    """Variation on the depth first search algorithm which sums up the edge values as
    it traverses the graph, and doesn't stop until all reachable nodes have been
    visited, returning all possible paths."""

    visited = set()
    all_paths = []

    def dfs(graph, node, path, path_sum):  # function for dfs

        path.append(node)

        if node == finish_node:
            all_paths.append((list(path), path_sum))

        if node not in visited:
            visited.add(node)
            for neighbour, delay in graph[node].items():
                dfs(graph, neighbour, path, path_sum+delay)

        path.pop()

    dfs(adjacency_list, start_node, path=[], path_sum=0)

    return all_paths
