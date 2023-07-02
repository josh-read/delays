from collections import defaultdict


def transpose_nested_dict(input_dict):
    """Swaps the first two keys of a nested dictionary."""
    output_dict = defaultdict(dict)
    for key_1, nested_dict in input_dict.items():
        for key_2, value in nested_dict.items():
            output_dict[key_2][key_1] = value
    return output_dict
