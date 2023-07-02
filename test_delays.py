from delays import DelayManager


def test_single_route():
    dm = DelayManager.from_list([
        ('e1', 'tb1', 'tb2', 100),
        ('e1', 'tb2', 'tb3', 25),
        ('e1', 'tb4', 'tb3', 250),
    ])
    delay = dm.find_delay('e1', 'tb1', 'tb4')
    assert delay == -125
